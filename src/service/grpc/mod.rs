pub mod error;
pub mod grpc {
    tonic::include_proto!("centurion");
}

use crate::repository::TaskBucketRepository;
use crate::service::grpc::grpc::task_service_server::TaskService;
use crate::service::grpc::grpc::{
    CreateBucketRequest, CreateBucketResponse, GetBucketByIdResponse, GetBucketsRequest,
    GetBucketsResponse,
};
use async_trait::async_trait;
use tonic::{Request, Response, Status};

pub struct TaskServiceImpl<TaskBucketRepositoryT: TaskBucketRepository> {
    _task_bucket_repository: TaskBucketRepositoryT,
}

impl<TaskRepositoryT: TaskBucketRepository> TaskServiceImpl<TaskRepositoryT> {
    pub fn new(task_repository: TaskRepositoryT) -> Self {
        Self {
            _task_bucket_repository: task_repository,
        }
    }
}

#[async_trait]
impl<TaskRepositoryT: TaskBucketRepository> TaskService for TaskServiceImpl<TaskRepositoryT> {
    async fn create_bucket(
        &self,
        _request: Request<CreateBucketRequest>,
    ) -> Result<Response<CreateBucketResponse>, Status> {
        todo!()
    }

    async fn get_bucket_by_id(
        &self,
        _request: Request<crate::service::grpc::CreateBucketRequest>,
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
