use uuid::Uuid;

trait IdService {
    fn generate(&self) -> String;
}

struct UuidIdService;

impl IdService for UuidIdService {
    fn generate(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
