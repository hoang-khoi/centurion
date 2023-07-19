use crate::model::aggregate::TaskBucket;
use crate::service::grpc::grpc::task_service_server::TaskService;
use crate::service::grpc::grpc::{CreateBucketRequest, GetBucketByIdRequest};
use crate::service::grpc::test::utils::setup_service;
use tonic::Request;

#[tokio::test]
async fn create_get_get_by_id() {
    let service = setup_service();
    let request = Request::new(CreateBucketRequest {
        name: "New Bucket".to_string(),
    });

    // First, create the bucket
    service.create_bucket(request).await.unwrap();

    // Get the list of buckets;
    let buckets: Vec<TaskBucket> = service
        .get_buckets(Request::new(()))
        .await
        .unwrap()
        .into_inner()
        .buckets
        .into_iter()
        .map(|t| TaskBucket::new(t.id, t.name))
        .collect();

    assert_eq!(1, buckets.len());

    let bucket_from_get_buckets = buckets[0].clone();
    assert_eq!("New Bucket", bucket_from_get_buckets.name());

    // Test get bucket by id
    let retrieved_bucket = service
        .get_bucket_by_id(Request::new(GetBucketByIdRequest {
            id: bucket_from_get_buckets.id().to_string(),
        }))
        .await
        .unwrap()
        .into_inner()
        .bucket
        .unwrap();

    assert_eq!(
        bucket_from_get_buckets,
        TaskBucket::new(retrieved_bucket.id, retrieved_bucket.name)
    );
}

mod utils {
    use crate::repository::stub::StubTaskRepository;
    use crate::service::grpc::TaskServiceImpl;
    use crate::service::id::UuidIdService;

    pub fn setup_service() -> TaskServiceImpl<StubTaskRepository, UuidIdService> {
        TaskServiceImpl::new(StubTaskRepository::new(), UuidIdService {})
    }
}
