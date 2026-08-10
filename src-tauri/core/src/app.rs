use surrealdb::{
    Surreal,
    engine::local::{Db, Mem, SurrealKv},
    types::RecordId,
};

use std::path::Path;

use crate::{
    error::SikshyaaError,
    models::{source::Source, video::Video},
};

pub struct SikshyaaApp {
    db: surrealdb::Surreal<Db>,
}

impl SikshyaaApp {
    pub async fn with_file_surreal(path: impl AsRef<Path>) -> Result<Self, SikshyaaError> {
        let db = Surreal::new::<SurrealKv>(path.as_ref()).await?;
        db.use_ns("sikshyaa").use_db("main").await?;
        Ok(Self { db })
    }

    pub async fn with_memory_surreal() -> Result<Self, SikshyaaError> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("sikshyaa").use_db("main").await?;
        Ok(Self { db })
    }

    // video methods
    pub async fn create_video(&self, video: Video) -> Result<Video, SikshyaaError> {
        tracing::debug!(subject = %video.subject, topic = %video.topic, "creating video");
        let created: Option<Video> = self.db.create("video").content(video).await?;
        let created = created.ok_or(SikshyaaError::VideoNotCreated)?;
        tracing::info!(subject = %created.subject, topic = %created.topic, "video created");
        Ok(created)
    }

    //source methods
    //
    //
    pub async fn create_source(&self, source: Source) -> Result<Source, SikshyaaError> {
        tracing::debug!(path = %source.path ,pattern = %source.pattern, "creating source");

        // is the pattern valid AND does the path provided in the pattern exist?
        let path = Path::new(&source.path);

        if !path.is_dir() || !path.exists() {
            tracing::error!(path = %source.path, "path does not exist");
            return Err(SikshyaaError::InvalidVideoDirectory);
        }

        let created_source: Option<Source> = self.db.create("source").content(source).await?;
        let created_source = created_source.ok_or(SikshyaaError::SourceNotCreated)?;
        Ok(created_source)
    }

    pub async fn edit_source(
        &self,
        source_id: RecordId,
        source: Source,
    ) -> Result<Source, SikshyaaError> {
        let path = Path::new(&source.path);

        if !path.is_dir() || !path.exists() {
            tracing::error!(path = %source.path, "path does not exist");
            return Err(SikshyaaError::InvalidVideoDirectory);
        }

        let updated_source: Option<Source> = self.db.update(source_id).content(source).await?;
        let updated_source = updated_source.ok_or(SikshyaaError::SourceNotCreated)?;

        Ok(updated_source)
    }

    pub async fn delete_source(&self, source_id: RecordId) -> Result<(), SikshyaaError> {
        let deleted: Option<Source> = self.db.delete(source_id).await?;

        if deleted.is_none() {
            return Err(SikshyaaError::SourceNotCreated);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::models::source;

    use super::*;

    #[tokio::test]
    async fn create_video_persists_and_returns_video() -> Result<(), SikshyaaError> {
        let app = SikshyaaApp::with_memory_surreal().await?;
        let video = Video::new("10", "Science", "Light", "Reflection")
            .with_teacher_name("Ada")
            .with_source("https://example.com/light");

        let created = app.create_video(video).await?;

        assert_eq!(created.grade, "10");
        assert_eq!(created.subject, "Science");
        assert_eq!(created.topic, "Light");
        assert_eq!(created.sub_topic, "Reflection");
        assert_eq!(created.teacher_name.as_deref(), Some("Ada"));
        assert_eq!(created.source.as_deref(), Some("https://example.com/light"));

        Ok(())
    }

    #[tokio::test]
    async fn create_source_persists_and_returns_source() -> Result<(), SikshyaaError> {
        let app = SikshyaaApp::with_memory_surreal().await?;

        let path = env::current_dir()
            .unwrap_or("C:\\".into())
            .display()
            .to_string();

        let pattern = "{{teacherName}}/{{grade}}/{{subject}}/{{topic}}/{{subTopic}}/{{grade?}}.mp4";

        let source = Source::new(path.to_string(), pattern.to_string());

        let created = app.create_source(source).await?;

        assert_eq!(created.path(), path);
        assert_eq!(created.pattern(), pattern);

        Ok(())
    }

    #[tokio::test]
    async fn edit_source_fails_on_invalid_directory() -> Result<(), SikshyaaError> {
        let app = SikshyaaApp::with_memory_surreal().await?;

        let path = env::current_dir()
            .unwrap_or("C:\\".into())
            .display()
            .to_string();

        let src = app
            .create_source(Source {
                id: Some(RecordId::new(source::SOURCE_TABLENAME, "random-id")),
                path,
                pattern:
                    "{{teacherName}}/{{grade}}/{{subject}}/{{topic}}/{{subTopic}}/{{grade?}}.mp4"
                        .to_string(),
            })
            .await;

        assert!(src.is_ok());

        let src = src.unwrap();

        let edit_source_result = app
            .edit_source(
                src.id.unwrap(),
                Source {
                    id: None,
                    path: "/path/that/does/not/exist/9999".to_string(),
                    pattern: "".to_string(),
                },
            )
            .await;

        assert!(edit_source_result.is_err());
        assert!(matches!(
            edit_source_result,
            Err(SikshyaaError::InvalidVideoDirectory)
        ));

        Ok(())
    }

    #[tokio::test]
    async fn edit_source_success() -> Result<(), SikshyaaError> {
        let app = SikshyaaApp::with_memory_surreal().await?;

        let initial_path = ".".to_string();
        let record_id = RecordId::new(source::SOURCE_TABLENAME, "test-id-123");
        let pattern = "{{teacherName}}/{{grade}}/{{subject}}/{{topic}}/{{subTopic}}/{{grade?}}.mp4";

        let src = app
            .create_source(Source {
                id: Some(record_id),
                path: initial_path,
                pattern: pattern.into(),
            })
            .await?;

        let updated_path_buf = std::env::current_dir().unwrap().join("test_temp_dir_edit");
        tokio::fs::create_dir_all(&updated_path_buf).await.ok();

        let updated_path = updated_path_buf.to_string_lossy().into_owned();
        let updated_pattern = pattern.to_string();
        let result = app
            .edit_source(
                src.id.clone().unwrap(),
                Source {
                    id: None,
                    path: updated_path.clone(),
                    pattern: updated_pattern.clone(),
                },
            )
            .await;

        let _ = tokio::fs::remove_dir(&updated_path_buf).await;

        let updated_source = result?;
        assert_eq!(updated_source.id, src.id);
        assert_eq!(updated_source.path, updated_path);
        assert_eq!(updated_source.pattern, updated_pattern);

        Ok(())
    }
    #[tokio::test]
    async fn create_and_delete_source_success() -> Result<(), SikshyaaError> {
        let app = SikshyaaApp::with_memory_surreal().await?;

        let record_id = RecordId::new(source::SOURCE_TABLENAME, "test-delete-id");
        let created_source = app
            .create_source(Source {
                id: Some(record_id.clone()),
                path: ".".to_string(),
                pattern: "{{teacherName}}/{{grade}}/{{subject}}.mp4".to_string(),
            })
            .await?;

        let source_id = created_source.id.expect("Source ID should be populated");

        app.delete_source(source_id.clone()).await?;

        let deleted_record: Option<Source> = app.db.select(source_id.clone()).await?;
        assert!(deleted_record.is_none());

        let second_delete_result = app.delete_source(source_id).await;
        assert!(matches!(
            second_delete_result,
            Err(SikshyaaError::SourceNotCreated)
        ));

        Ok(())
    }
}
