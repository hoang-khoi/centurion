use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ModelError {
    #[error("Invalid value object")]
    InvalidValueObject,
}
