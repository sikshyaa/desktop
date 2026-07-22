use surrealdb::{
    Surreal,
    engine::local::{Db, Mem, SurrealKv},
};

use crate::{error::SikshyaaError, models::video::Video};

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
        let created: Option<Video> = self.db.create("video").content(video).await?;
        created.ok_or(SikshyaaError::VideoNotCreated)
    }

    //source methods
    //
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_video_persists_and_returns_video() -> Result<(), SikshyaaError> {
        let app = SikshyaaApp::with_memory_surreal().await?;
        let video = Video {
            grade: "10".to_owned(),
            subject: "Science".to_owned(),
            topic: "Light".to_owned(),
            sub_topic: "Reflection".to_owned(),
            teacher_name: Some("Ada".to_owned()),
            source: Some("https://example.com/light".to_owned()),
        };

        let created = app.create_video(video).await?;

        assert_eq!(created.grade, "10");
        assert_eq!(created.subject, "Science");
        assert_eq!(created.topic, "Light");
        assert_eq!(created.sub_topic, "Reflection");
        assert_eq!(created.teacher_name.as_deref(), Some("Ada"));
        assert_eq!(created.source.as_deref(), Some("https://example.com/light"));

        Ok(())
    }
}
