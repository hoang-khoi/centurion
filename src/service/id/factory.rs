use crate::service::id::UuidIdService;

pub type IdServiceType = UuidIdService;

pub fn id_service() -> IdServiceType {
    UuidIdService {}
}
