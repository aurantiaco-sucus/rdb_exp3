mod model;
mod server;
mod client;

#[tokio::main]
async fn main() {
    let lms_launch_type = std::env::var("lms_launch_type")
        .unwrap_or_else(|_| "client".to_string());
    let lms_host = std::env::var("lms_host")
        .unwrap_or_else(|_| "localhost".to_string());
    let lms_port = std::env::var("lms_port")
        .unwrap_or_else(|_| "9998".to_string());
    match lms_launch_type.as_str() {
        "server" => server::main_server(lms_port).await,
        "client" => client::main_client(lms_host, lms_port).await,
        _ => panic!("Unknown launch type: {}", lms_launch_type),
    }
}