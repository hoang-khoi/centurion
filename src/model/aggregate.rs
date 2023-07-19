use crate::service::grpc::grpc;

#[derive(Debug, Clone)]
pub struct TaskBucket {
    id: String,
    name: String,
}

impl TaskBucket {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<TaskBucket> for grpc::TaskBucket {
    fn from(bucket: TaskBucket) -> Self {
        Self {
            id: bucket.id,
            name: bucket.name,
        }
    }
}
