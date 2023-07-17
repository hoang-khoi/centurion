use uuid::Uuid;

pub trait IdService {
    fn generate(&self) -> String;
}

pub struct UuidIdService;

impl IdService for UuidIdService {
    fn generate(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
