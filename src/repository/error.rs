use thiserror::Error;
use tonic::{Code, Status};

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Internal repository error")]
    Internal,

    #[error("Resource not found")]
    NotFound,
}

impl From<&RepositoryError> for Status {
    fn from(error: &RepositoryError) -> Self {
        let message = format!("{}", error);

        match error {
            RepositoryError::Internal => Status::new(Code::Internal, message),
            RepositoryError::NotFound => Status::new(Code::NotFound, message),
        }
    }
}
