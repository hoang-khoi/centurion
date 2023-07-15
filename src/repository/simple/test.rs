use crate::model::aggregate::TaskBucket;
use crate::repository::simple::test::utils::setup_repository;
use crate::repository::TaskRepository;

#[tokio::test]
async fn save_and_get_by_id() {
    let repository = setup_repository();

    let bucket = TaskBucket::new(
        "751E367D-D586-4162-84F8-4690AF1EF796".to_string(),
        "Finish this project".to_string(),
    )
    .unwrap();

    repository.save(&bucket).await.unwrap();

    let retrieved_bucket = repository.get_by_id(bucket.id()).await.unwrap();

    assert_eq!(bucket, retrieved_bucket);
}

mod utils {
    use crate::repository::simple::SimpleTaskRepository;

    pub fn setup_repository() -> SimpleTaskRepository {
        SimpleTaskRepository::new()
    }
}
