pub mod error;
pub mod stub;

use crate::model::aggregate::TaskBucket;
use crate::repository::error::RepositoryError;
use async_trait::async_trait;
use error_stack::Report;

#[async_trait]
pub trait TaskBucketRepository: Send + Sync + 'static {
    async fn save(&self, task_bucket: &TaskBucket) -> Result<(), Report<RepositoryError>>;
    async fn get_by_id(&self, id: &str) -> Result<TaskBucket, Report<RepositoryError>>;
    async fn get_all(&self) -> Result<Vec<TaskBucket>, Report<RepositoryError>>;
}
