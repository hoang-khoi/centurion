#[cfg(test)]
mod test;

use crate::model::aggregate::TaskBucket;
use crate::repository::error::RepositoryError;
use crate::repository::TaskRepository;
use async_trait::async_trait;
use error_stack::Report;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SimpleTaskRepository {
    bucket_map: Arc<Mutex<HashMap<String, TaskBucket>>>,
}

impl SimpleTaskRepository {
    pub fn new() -> Self {
        let bucket_map = Arc::new(Mutex::new(HashMap::new()));
        Self { bucket_map }
    }
}

#[async_trait]
impl TaskRepository for SimpleTaskRepository {
    async fn save(&self, task_bucket: &TaskBucket) -> Result<(), Report<RepositoryError>> {
        self.bucket_map
            .clone()
            .lock()
            .unwrap()
            .insert(task_bucket.id().to_string(), task_bucket.clone());

        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> Result<TaskBucket, Report<RepositoryError>> {
        self.bucket_map
            .clone()
            .lock()
            .unwrap()
            .get(id)
            .ok_or(Report::new(RepositoryError::NotFound))
            .cloned()
    }
}
