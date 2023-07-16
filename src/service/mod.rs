pub mod error;
#[cfg(test)]
mod test;

use crate::model::aggregate::TaskBucket;
use crate::repository::TaskRepository;
use crate::service::error::ServiceError;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};

#[async_trait]
trait TaskService {
    async fn create_bucket(&self, bucket: &TaskBucket) -> Result<(), Report<ServiceError>>;
}

pub struct TaskServiceImpl<TaskRepositoryT: TaskRepository> {
    task_repository: TaskRepositoryT,
}

impl<TaskRepositoryT: TaskRepository> TaskServiceImpl<TaskRepositoryT> {
    pub fn new(task_repository: TaskRepositoryT) -> Self {
        Self { task_repository }
    }
}

#[async_trait]
impl<TaskRepositoryT: TaskRepository> TaskService for TaskServiceImpl<TaskRepositoryT> {
    async fn create_bucket(&self, bucket: &TaskBucket) -> Result<(), Report<ServiceError>> {
        self.task_repository
            .save(bucket)
            .await
            .change_context(ServiceError::Internal)
    }
}
