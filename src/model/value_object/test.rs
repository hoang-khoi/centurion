mod create_bucket_request {
    use crate::model::error::ModelError;
    use crate::model::value_object::CreateBucketRequest;

    #[test]
    fn invalid_names() {
        let names = [
            "123",  // Too short
            "123456789012345678901234567890123456789012345678901"   // Too long
        ];

        for name in names {
            let request = CreateBucketRequest::new(name.to_string());
            assert!(request.is_err_and(|e| e.current_context() == &ModelError::InvalidValueObject));
        }
    }

    #[test]
    fn valid_names() {
        let names = [
            "1234",  // Minimal length
            "12345678901234567890123456789012345678901234567890"   // Maximal length
        ];

        for name in names {
            let request = CreateBucketRequest::new(name.to_string());
            assert!(request.is_ok());
        }
    }
}

