use std::{convert::Infallible, net::SocketAddr, str::FromStr};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Server};

use crate::proxy;

pub async fn start_server(ip: String) {
    let addr = match SocketAddr::from_str(&ip) {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let backend_server: hyper::Uri = "http://localhost:8080/ola".parse().unwrap(); // Parse backend_server as hyper::Uri

    let client = Client::new();

    let make_svc = make_service_fn(move |_| {
        let client = client.clone();
        let backend_server = backend_server.clone();

        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                proxy::handle_proxy_request(req, client.clone(), backend_server.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
