use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Internal repository error")]
    Internal,

    #[error("Resource not found")]
    NotFound,
}
