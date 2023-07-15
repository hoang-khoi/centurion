use crate::model::aggregate::TaskBucket;
use async_trait::async_trait;

pub mod simple;

#[async_trait]
pub trait TaskBucketRepository {
    async fn save(&self, task_bucket: &TaskBucket);
}
