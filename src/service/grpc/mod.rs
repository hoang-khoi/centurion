pub mod error;
pub mod grpc {
    tonic::include_proto!("centurion");
}
pub mod factory;

use crate::model::aggregate::TaskBucket;
use crate::model::error::ModelError;
use crate::model::value_object::{ParsedCreateBucketRequest, ParsedGetBucketByIdRequest};
use crate::repository::TaskBucketRepository;
use crate::service::grpc::grpc::task_service_server::TaskService;
use crate::service::grpc::grpc::{
    CreateBucketRequest, GetBucketByIdRequest, GetBucketByIdResponse, GetBucketsRequest,
    GetBucketsResponse,
};
use crate::service::id::IdService;
use async_trait::async_trait;
use error_stack::Report;
use tonic::{Request, Response, Status};

pub struct TaskServiceImpl<TaskBucketRepositoryT, IdServiceT>
where
    TaskBucketRepositoryT: TaskBucketRepository,
    IdServiceT: IdService,
{
    task_bucket_repository: TaskBucketRepositoryT,
    id_service: IdServiceT,
}

impl<TaskRepositoryT, IdServiceT> TaskServiceImpl<TaskRepositoryT, IdServiceT>
where
    TaskRepositoryT: TaskBucketRepository,
    IdServiceT: IdService,
{
    pub fn new(task_bucket_repository: TaskRepositoryT, id_service: IdServiceT) -> Self {
        Self {
            task_bucket_repository,
            id_service,
        }
    }
}

#[async_trait]
impl<TaskRepositoryT, IdServiceT> TaskService for TaskServiceImpl<TaskRepositoryT, IdServiceT>
where
    TaskRepositoryT: TaskBucketRepository,
    IdServiceT: IdService,
{
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

        let bucket = TaskBucket::new(
            self.id_service.generate(),
            parsed_request.name().to_string(),
        );

        self.task_bucket_repository.create(&bucket).await.unwrap();

        Ok(Response::new(()))
    }

    async fn get_bucket_by_id(
        &self,
        request: Request<GetBucketByIdRequest>,
    ) -> Result<Response<GetBucketByIdResponse>, Status> {
        let parsed_request: ParsedGetBucketByIdRequest = request.into();
        let bucket = self
            .task_bucket_repository
            .get_by_id(&parsed_request.id)
            .await
            .map_err(|e| {
                let status: Status = e.current_context().into();
                status
            })?;

        Ok(Response::new(GetBucketByIdResponse {
            bucket: Some(bucket.into()),
        }))
    }

    async fn get_buckets(
        &self,
        _request: Request<GetBucketsRequest>,
    ) -> Result<Response<GetBucketsResponse>, Status> {
        todo!()
    }
}
