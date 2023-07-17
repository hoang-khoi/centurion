pub mod error;
pub mod stub;

use crate::model::aggregate::TaskBucket;
use crate::model::value_object::ParsedCreateBucketRequest;
use crate::repository::error::RepositoryError;
use async_trait::async_trait;
use error_stack::Report;

#[async_trait]
pub trait TaskBucketRepository: Send + Sync + 'static {
    async fn create(
        &self,
        request: ParsedCreateBucketRequest,
    ) -> Result<(), Report<RepositoryError>>;
    async fn get_by_id(&self, id: &str) -> Result<TaskBucket, Report<RepositoryError>>;
    async fn get_all(&self) -> Result<Vec<TaskBucket>, Report<RepositoryError>>;
}
