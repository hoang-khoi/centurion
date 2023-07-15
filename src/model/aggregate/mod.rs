#[cfg(test)]
mod test;

use crate::model::error::ModelError;
use error_stack::{IntoReport, Report, ResultExt};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct TaskBucket {
    #[validate(length(equal = 36))]
    id: String,
    #[validate(length(min = 4, max = 50))]
    name: String,
}

impl TaskBucket {
    pub fn new(id: String, name: String) -> Result<Self, Report<ModelError>> {
        let bucket = TaskBucket { id, name };

        bucket
            .validate()
            .into_report()
            .change_context(ModelError::InvalidValueObject)?;

        Ok(bucket)
    }
}
