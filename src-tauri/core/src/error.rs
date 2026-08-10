use thiserror::Error;

#[derive(Debug, Error)]
pub enum SikshyaaError {
    #[error("database error {0}")]
    Database(#[from] surrealdb::Error),
    #[error("video was not created")]
    VideoNotCreated,

    #[error("source was not created")]
    SourceNotCreated,

    #[error("invalid directory passed")]
    InvalidVideoDirectory,
}
