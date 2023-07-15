use crate::model::aggregate::TaskBucket;
use crate::repository::simple::error::RepositoryError;
use async_trait::async_trait;
use error_stack::Report;

pub mod simple;

#[async_trait]
pub trait TaskBucketRepository {
    async fn save(&self, task_bucket: &TaskBucket) -> Result<(), Report<RepositoryError>>;
}
