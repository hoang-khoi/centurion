use crate::service::grpc::factory::{task_service_impl, TaskServiceType};
use crate::service::grpc::grpc::task_service_server::TaskServiceServer;
use std::any::type_name;
use std::env::var;
use std::net::SocketAddr;
use std::str::FromStr;

const SERVER_HOST: &str = "SERVER_HOST";
const SERVER_PORT: &str = "SERVER_PORT";

const DEFAULT_SERVER_HOST: &str = "0.0.0.0";
const DEFAULT_SERVER_PORT: &str = "8080";

pub fn task_service_server() -> TaskServiceServer<TaskServiceType> {
    TaskServiceServer::new(task_service_impl())
}

pub fn socket_addr() -> SocketAddr {
    SocketAddr::new(
        get_env_var_or_default(SERVER_HOST, DEFAULT_SERVER_HOST),
        get_env_var_or_default(SERVER_PORT, DEFAULT_SERVER_PORT),
    )
}

fn get_env_var_or_default<T: FromStr>(key: &str, default: &str) -> T {
    var(key)
        .unwrap_or(default.to_string())
        .parse::<T>()
        .unwrap_or_else(|_| {
            panic!(
                "Failed to parse env var {}, expected: {}",
                key,
                type_name::<T>()
            )
        })
}
