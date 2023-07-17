use crate::model::aggregate::TaskBucket;
use crate::model::value_object::ParsedCreateBucketRequest;
use crate::repository::error::RepositoryError;
use crate::repository::TaskBucketRepository;
use crate::service::id::{IdService, UuidIdService};
use async_trait::async_trait;
use error_stack::Report;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct StubTaskRepository<IdServiceT: IdService> {
    bucket_map: Arc<Mutex<HashMap<String, TaskBucket>>>,
    id_service: Arc<Mutex<IdServiceT>>,
}

impl StubTaskRepository<UuidIdService> {
    pub fn new() -> Self {
        let bucket_map = Arc::new(Mutex::new(HashMap::new()));
        Self {
            bucket_map,
            id_service: Arc::new(Mutex::new(UuidIdService {})),
        }
    }
}

#[async_trait]
impl TaskBucketRepository for StubTaskRepository<UuidIdService> {
    async fn create(
        &self,
        request: ParsedCreateBucketRequest,
    ) -> Result<(), Report<RepositoryError>> {
        let bucket = TaskBucket::new(
            self.id_service.clone().lock().unwrap().generate(),
            request.name().to_string(),
        );

        self.bucket_map
            .clone()
            .lock()
            .unwrap()
            .insert(bucket.id().to_string(), bucket);

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

    async fn get_all(&self) -> Result<Vec<TaskBucket>, Report<RepositoryError>> {
        let buckets: Vec<TaskBucket> = self
            .bucket_map
            .clone()
            .lock()
            .unwrap()
            .iter()
            .map(|(_, bucket)| bucket.clone())
            .collect();

        Ok(buckets)
    }
}
