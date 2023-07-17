use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Internal service error")]
    Internal,
}
