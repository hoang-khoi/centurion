use thiserror::Error;
use tonic::{Code, Status};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ModelError {
    #[error("Invalid value object")]
    InvalidValueObject,
}

impl From<&ModelError> for Status {
    fn from(error: &ModelError) -> Self {
        let message = format!("{}", error);

        match error {
            ModelError::InvalidValueObject => Status::new(Code::InvalidArgument, message),
        }
    }
}
