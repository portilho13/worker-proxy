use std::net::IpAddr;
use std::str::FromStr;

use hyper::{Body, Client, Request, Response, StatusCode};
use hyper::client::HttpConnector;

use crate::ddos::ratelimiter::{self, get_token_bucket, TokenBucket};

pub async fn handle_proxy_request(
    req: Request<Body>,
    client: Client<HttpConnector>,
    backend_server: hyper::Uri,
    addr: IpAddr
) -> Result<Response<Body>, hyper::Error> {
    let allow_request = {
        let mut buckets = get_token_bucket(addr);
        let bucket = buckets.get_mut(&addr).unwrap();
        bucket.allow_request(1.0)
    };

    if allow_request {
        let proxied_request = Request::builder()
            .method(req.method().clone())
            .uri(backend_server.clone())
            .body(req.into_body())
            .expect("Failed to build request");
    
        client.request(proxied_request).await
    } else {
        let mut response = Response::new(Body::from("Too many requests"));
        *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
        Ok(response)
    }
}
