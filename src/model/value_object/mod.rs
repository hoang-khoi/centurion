use crate::model::error::ModelError;
use crate::service::grpc::grpc::{CreateBucketRequest, GetBucketByIdRequest};
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

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl TryFrom<Request<CreateBucketRequest>> for ParsedCreateBucketRequest {
    type Error = Report<ModelError>;

    fn try_from(grpc_request: Request<CreateBucketRequest>) -> Result<Self, Self::Error> {
        let payload = grpc_request.into_inner();
        ParsedCreateBucketRequest::new(payload.name)
    }
}

pub struct ParsedGetBucketByIdRequest {
    pub id: String,
}

impl From<Request<GetBucketByIdRequest>> for ParsedGetBucketByIdRequest {
    fn from(request: Request<GetBucketByIdRequest>) -> Self {
        Self {
            id: request.into_inner().id,
        }
    }
}

#[cfg(test)]
mod test;
