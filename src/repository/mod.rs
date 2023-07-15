use crate::model::aggregate::TaskBucket;
use async_trait::async_trait;

#[async_trait]
trait TaskBucketRepository {
    async fn create_or_update(&self, task_bucket: &TaskBucket);
}
