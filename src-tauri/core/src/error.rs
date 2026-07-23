use thiserror::Error;

#[derive(Debug, Error)]
pub enum SikshyaaError {
    #[error("database error {0}")]
    Database(#[from] surrealdb::Error),
    #[error("video was not created")]
    VideoNotCreated,
}
