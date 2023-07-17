use crate::model::value_object::ParsedCreateBucketRequest;

#[test]
fn invalid_create_bucket_request() {
    let invalid_requests = [
        ParsedCreateBucketRequest::new("123".to_string()),
        ParsedCreateBucketRequest::new(
            "12345678901234567890123456789012345678901234567890123456789012345".to_string(),
        ),
    ];

    for request in invalid_requests {
        assert!(request.is_err());
    }
}

#[test]
fn valid_create_bucket_request() {
    let invalid_requests = [
        ParsedCreateBucketRequest::new("1234".to_string()),
        ParsedCreateBucketRequest::new(
            "1234567890123456789012345678901234567890123456789012345678901234".to_string(),
        ),
    ];

    for request in invalid_requests {
        assert!(request.is_ok());
    }
}
