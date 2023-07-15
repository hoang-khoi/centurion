use crate::model::aggregate::TaskBucket;
use crate::service::error::ServiceError;
use async_trait::async_trait;
use error_stack::Report;

pub mod error;

#[async_trait]
trait TaskService {
    async fn create_bucket(&self, bucket: TaskBucket) -> Result<(), Report<ServiceError>>;
}

struct TaskServiceImpl;

#[async_trait]
impl TaskService for TaskServiceImpl {
    async fn create_bucket(&self, _bucket: TaskBucket) -> Result<(), Report<ServiceError>> {
        todo!()
    }
}
