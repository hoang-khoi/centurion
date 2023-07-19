use crate::factory::{socket_addr, task_service_server};
use tonic::transport::Server;

pub mod factory;
pub mod model;
pub mod repository;
pub mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    Server::builder()
        .add_service(task_service_server())
        .serve(socket_addr())
        .await?;

    Ok(())
}
