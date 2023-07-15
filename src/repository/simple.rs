use crate::model::aggregate::TaskBucket;
use crate::repository::TaskBucketRepository;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SimpleTaskBucketRepository {
    bucket_map: Arc<Mutex<HashMap<String, TaskBucket>>>,
}

impl SimpleTaskBucketRepository {
    pub fn new() -> Self {
        let bucket_map = Arc::new(Mutex::new(HashMap::new()));
        Self { bucket_map }
    }
}

#[async_trait]
impl TaskBucketRepository for SimpleTaskBucketRepository {
    async fn save(&self, task_bucket: &TaskBucket) {
        self.bucket_map
            .clone()
            .lock()
            .unwrap()
            .insert(task_bucket.id().to_string(), task_bucket.clone());
    }
}
