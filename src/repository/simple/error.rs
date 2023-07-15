use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Internal repository error")]
    Internal,
}
