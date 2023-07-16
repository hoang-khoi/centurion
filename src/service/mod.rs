pub mod error;
#[cfg(test)]
mod test;

use crate::model::aggregate::TaskBucket;
use crate::repository::TaskBucketRepository;
use crate::service::error::ServiceError;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};

#[async_trait]
trait TaskService {
    async fn create_bucket(&self, bucket: &TaskBucket) -> Result<(), Report<ServiceError>>;
    async fn get_bucket_by_id(&self, id: &str) -> Result<TaskBucket, Report<ServiceError>>;
    async fn get_buckets(&self) -> Result<Vec<TaskBucket>, Report<ServiceError>>;
}

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
    async fn create_bucket(&self, bucket: &TaskBucket) -> Result<(), Report<ServiceError>> {
        self.task_bucket_repository
            .save(bucket)
            .await
            .change_context(ServiceError::Internal)
    }

    async fn get_bucket_by_id(&self, id: &str) -> Result<TaskBucket, Report<ServiceError>> {
        self.task_bucket_repository
            .get_by_id(id)
            .await
            .change_context(ServiceError::Internal)
    }

    async fn get_buckets(&self) -> Result<Vec<TaskBucket>, Report<ServiceError>> {
        self.task_bucket_repository
            .get_all()
            .await
            .change_context(ServiceError::Internal)
    }
}
