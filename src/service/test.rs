use crate::model::aggregate::TaskBucket;
use crate::service::test::utils::setup_service;
use crate::service::TaskService;

#[tokio::test]
async fn create_bucket_get_bucket() {
    let service = setup_service();
    let bucket = TaskBucket::new(
        "751E367D-D586-4162-84F8-4690AF1EF796".to_string(),
        "Productive task".to_string(),
    )
    .unwrap();

    service.create_bucket(&bucket).await.unwrap();

    let retrieved_bucket = service.get_bucket_by_id(bucket.id()).await.unwrap();

    assert_eq!(bucket, retrieved_bucket);
}

#[tokio::test]
async fn get_buckets() {
    let service = setup_service();

    let bucket = TaskBucket::new(
        "751E367D-D586-4162-84F8-4690AF1EF796".to_string(),
        "Productive task".to_string(),
    )
    .unwrap();
    service.create_bucket(&bucket).await.unwrap();

    let buckets = service.get_buckets().await.unwrap();

    assert_eq!(vec![bucket], buckets);
}

mod utils {
    use crate::repository::stub::StubTaskRepository;
    use crate::service::TaskServiceImpl;

    pub fn setup_service() -> TaskServiceImpl<StubTaskRepository> {
        TaskServiceImpl::new(StubTaskRepository::new())
    }
}
