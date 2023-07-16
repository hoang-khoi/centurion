use crate::model::aggregate::TaskBucket;
use crate::service::test::utils::setup_service;
use crate::service::TaskService;

mod stub;

#[tokio::test]
async fn create_bucket() {
    let service = setup_service();
    let bucket = TaskBucket::new(
        "751E367D-D586-4162-84F8-4690AF1EF796".to_string(),
        "Productive task".to_string(),
    )
    .unwrap();

    service.create_bucket(&bucket).await.unwrap();

    assert!(service.task_repository.is_bucket_saved(&bucket));
}

mod utils {
    use crate::service::test::stub::TaskRepositoryStub;
    use crate::service::TaskServiceImpl;

    pub fn setup_service() -> TaskServiceImpl<TaskRepositoryStub> {
        TaskServiceImpl::new(TaskRepositoryStub::new())
    }
}
