use error_stack::{IntoReport, Report, ResultExt};
use validator::{Validate};
use crate::model::error::ModelError;


#[derive(Validate, Debug)]
pub struct CreateBucketRequest {
    #[validate(length(min = 4, max = 50))]
    name: String
}

impl CreateBucketRequest {
    pub fn new(name: String) -> Result<Self, Report<ModelError>> {
        let request = Self { name };

        request.validate().into_report().change_context(ModelError::InvalidValueObject)?;

        Ok(request)
    }
}


#[cfg(test)]
mod test;
