use crate::repository::stub::StubTaskRepository;

pub type TaskBucketRepositoryType = StubTaskRepository;

pub fn task_bucket_repository() -> TaskBucketRepositoryType {
    StubTaskRepository::new()
}
