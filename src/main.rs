mod server;
mod proxy;
use tokio;

const SERVER_IP: &str = "127.0.0.1:8081";

#[tokio::main]
async fn main() {
    server::start_server(SERVER_IP.to_string()).await;
}
