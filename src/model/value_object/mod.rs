use crate::model::error::ModelError;
use crate::service::grpc::grpc::CreateBucketRequest;
use error_stack::{IntoReport, Report, ResultExt};
use tonic::Request;
use validator::Validate;

#[derive(Validate)]
pub struct ParsedCreateBucketRequest {
    #[validate(length(min = 4, max = 64))]
    name: String,
}

impl ParsedCreateBucketRequest {
    pub fn new(name: String) -> Result<Self, Report<ModelError>> {
        let request = Self { name };

        request
            .validate()
            .into_report()
            .change_context(ModelError::InvalidValueObject)?;

        Ok(request)
    }
}

impl TryFrom<Request<CreateBucketRequest>> for ParsedCreateBucketRequest {
    type Error = Report<ModelError>;

    fn try_from(grpc_request: Request<CreateBucketRequest>) -> Result<Self, Self::Error> {
        let payload = grpc_request.into_inner();
        ParsedCreateBucketRequest::new(payload.name)
    }
}

#[cfg(test)]
mod test;
