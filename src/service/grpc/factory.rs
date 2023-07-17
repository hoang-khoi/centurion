use crate::repository::factory::{task_bucket_repository, TaskBucketRepositoryType};
use crate::service::grpc::TaskServiceImpl;
use crate::service::id::factory::{id_service, IdServiceType};

pub type TaskServiceType = TaskServiceImpl<TaskBucketRepositoryType, IdServiceType>;

pub fn task_service_impl() -> TaskServiceType {
    TaskServiceImpl::new(task_bucket_repository(), id_service())
}
