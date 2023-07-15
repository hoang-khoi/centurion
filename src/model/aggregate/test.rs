mod task_bucket {
    use crate::model::aggregate::TaskBucket;
    use crate::model::error::ModelError;

    #[test]
    fn invalid_names() {
        let ids = [
            "123",                                                 // Too short
            "123456789012345678901234567890123456789012345678901", // Too long
        ];

        let dummy_id = "751E367D-D586-4162-84F8-4690AF1EF796";

        for name in ids {
            assert!(TaskBucket::new(dummy_id.to_string(), name.to_string())
                .is_err_and(|e| e.current_context() == &ModelError::InvalidValueObject));
        }
    }

    #[test]
    fn invalid_ids() {
        let ids = [
            "751E367D-D586-4162-84F8-4690AF1EF79",   // Too short
            "751E367D-D586-4162-84F8-4690AF1EF796X", // Too long
        ];

        let dummy_name = "bucket";

        for id in ids {
            assert!(TaskBucket::new(id.to_string(), dummy_name.to_string())
                .is_err_and(|e| e.current_context() == &ModelError::InvalidValueObject));
        }
    }

    #[test]
    fn valid_names_and_id() {
        let ids = [
            "1234",                                               // Barely enough
            "12345678901234567890123456789012345678901234567890", // Barely fits
        ];

        let dummy_id = "751E367D-D586-4162-84F8-4690AF1EF796";

        for id in ids {
            assert!(TaskBucket::new(dummy_id.to_string(), id.to_string()).is_ok());
        }
    }
}
