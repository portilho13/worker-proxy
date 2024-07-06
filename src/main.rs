mod server;
mod proxy;
mod ddos;

use std::{thread, time::Duration};

use tokio;

const SERVER_IP: &str = "127.0.0.1:8081";

#[tokio::main]
async fn main() {
    //server::start_server(SERVER_IP.to_string()).await;
    let mut obj = ddos::ratelimiter::TokenBucket::new(3.0, 0);
    println!("Allow Request: {}", obj.allow_request(1.0));
    println!("Allow Request: {}", obj.allow_request(1.0));
    println!("Allow Request: {}", obj.allow_request(1.0));
    println!("Allow Request: {}", obj.allow_request(1.0));
    println!("Allow Request: {}", obj.allow_request(1.0));
}
