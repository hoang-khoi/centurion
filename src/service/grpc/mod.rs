pub mod error;
pub mod grpc {
    tonic::include_proto!("centurion");
}

use crate::model::error::ModelError;
use crate::model::value_object::ParsedCreateBucketRequest;
use crate::repository::TaskBucketRepository;
use crate::service::grpc::grpc::task_service_server::TaskService;
use crate::service::grpc::grpc::{
    CreateBucketRequest, GetBucketByIdResponse, GetBucketsRequest, GetBucketsResponse,
};
use async_trait::async_trait;
use error_stack::Report;
use tonic::{Request, Response, Status};

pub struct TaskServiceImpl<TaskBucketRepositoryT: TaskBucketRepository> {
    task_bucket_repository: TaskBucketRepositoryT,
}

impl<TaskRepositoryT: TaskBucketRepository> TaskServiceImpl<TaskRepositoryT> {
    pub fn new(task_repository: TaskRepositoryT) -> Self {
        Self {
            task_bucket_repository: task_repository,
        }
    }
}

#[async_trait]
impl<TaskRepositoryT: TaskBucketRepository> TaskService for TaskServiceImpl<TaskRepositoryT> {
    async fn create_bucket(
        &self,
        request: Request<CreateBucketRequest>,
    ) -> Result<Response<()>, Status> {
        let parsed_request_result: Result<ParsedCreateBucketRequest, Report<ModelError>> =
            request.try_into();

        let parsed_request = parsed_request_result.map_err(|e| {
            let status: Status = e.current_context().into();
            status
        })?;

        self.task_bucket_repository
            .create(parsed_request)
            .await
            .unwrap();

        Ok(Response::new(()))
    }

    async fn get_bucket_by_id(
        &self,
        _request: Request<CreateBucketRequest>,
    ) -> Result<Response<GetBucketByIdResponse>, Status> {
        todo!()
    }

    async fn get_buckets(
        &self,
        _request: Request<GetBucketsRequest>,
    ) -> Result<Response<GetBucketsResponse>, Status> {
        todo!()
    }
}
