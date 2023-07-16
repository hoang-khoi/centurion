use crate::model::aggregate::TaskBucket;
use crate::repository::error::RepositoryError;
use crate::repository::TaskRepository;
use async_trait::async_trait;
use error_stack::Report;
use std::sync::{Arc, Mutex};

pub struct TaskRepositoryStub {
    saved_task_buckets: Arc<Mutex<Vec<TaskBucket>>>,
}

impl TaskRepositoryStub {
    pub fn new() -> Self {
        Self {
            saved_task_buckets: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn is_bucket_saved(&self, bucket: &TaskBucket) -> bool {
        self.saved_task_buckets.lock().unwrap().contains(bucket)
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryStub {
    async fn save(&self, task_bucket: &TaskBucket) -> Result<(), Report<RepositoryError>> {
        self.saved_task_buckets
            .clone()
            .lock()
            .unwrap()
            .push(task_bucket.clone());

        Ok(())
    }

    async fn get_by_id(&self, _id: &str) -> Result<TaskBucket, Report<RepositoryError>> {
        todo!()
    }
}
