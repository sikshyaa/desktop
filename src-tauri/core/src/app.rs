use surrealdb::{
    Surreal,
    engine::local::{Db, Mem, SurrealKv},
};

use crate::{
    error::SikshyaaError,
    models::{source::Source, video::Video},
};

pub struct SikshyaaApp {
    db: surrealdb::Surreal<Db>,
}

impl SikshyaaApp {
    pub async fn with_file_surreal(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, SikshyaaError> {
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

        let created_source: Option<Source> = self.db.create("source").content(source).await?;
        let created_source = created_source.ok_or(SikshyaaError::SourceNotCreated)?;
        Ok(created_source)
    }
}

#[cfg(test)]
mod tests {
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
        let path = "C:\\Users\\Aashutosh\\Videos";
        let pattern = "{{teacherName}}/{{grade}}/{{subject}}/{{topic}}/{{subTopic}}/{{grade?}}.mp4";

        let source = Source::new(path.to_string(), pattern.to_string());

        let created = app.create_source(source).await?;

        assert_eq!(created.path(), path);
        assert_eq!(created.pattern(), pattern);

        Ok(())
    }
}
