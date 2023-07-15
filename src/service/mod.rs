use error_stack::Report;
use crate::service::error::ServiceError;

pub mod error;

trait TaskService {
    fn create_bucket(&self, name: String) -> Result<(), Report<ServiceError>>;
}
