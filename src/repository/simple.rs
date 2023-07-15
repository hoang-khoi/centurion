use crate::model::aggregate::TaskBucket;
use crate::repository::TaskBucketRepository;
use async_trait::async_trait;

pub struct SimpleTaskBucketRepository;

#[async_trait]
impl TaskBucketRepository for SimpleTaskBucketRepository {
    async fn create_or_update(&self, _task_bucket: &TaskBucket) {
        todo!()
    }
}
