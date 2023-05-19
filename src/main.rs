mod model;
mod server;
mod client;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    let lms_launch_type = std::env::var("lms_launch_type")
        .unwrap_or_else(|_| "client".to_string());
    let lms_host = std::env::var("lms_host")
        .unwrap_or_else(|_| "localhost".to_string());
    let lms_port = std::env::var("lms_port")
        .unwrap_or_else(|_| "9998".to_string());
    let lms_config_overwrite = std::env::var("lms_config_overwrite")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .expect("lms_config_overwrite must be a boolean");
    match lms_launch_type.as_str() {
        "server" => server::main_server(lms_port).await,
        "client" => client::main_client(lms_host, lms_port).await,
        "config" => config::main_config(lms_config_overwrite).await,
        _ => panic!("Unknown launch type: {}", lms_launch_type),
    }
}