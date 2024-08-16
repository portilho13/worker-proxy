use std::net::IpAddr;
use std::str::FromStr;

use hyper::{Body, Client, Request, Response, StatusCode};
use hyper::client::HttpConnector;

use crate::ddos::ratelimiter::{self, get_token_bucket, TokenBucket};

pub async fn handle_proxy_request(
    mut req: Request<Body>,
    client: Client<HttpConnector>,
    backend_server: hyper::Uri,
    addr: IpAddr
) -> Result<Response<Body>, hyper::Error> {
    let allow_request = {
        let mut buckets = get_token_bucket(addr);
        let bucket = buckets.get_mut(&addr).unwrap();
        bucket.allow_request(1.0)
    };

    println!("Request from IP: {}", addr);

    if allow_request {
        println!("Allowing Request from IP {} to {}", addr, backend_server);

/*         let uri_string = format!(
            "http://{}{}",
            backend_server.authority().unwrap(),
            req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
        );

        let uri = uri_string.parse::<hyper::Uri>().unwrap();

        let proxied_request = Request::builder()
            .method(req.method().clone())
            .uri(uri.clone())
            .body(req.into_body())
            .expect("Failed to build request");
    
        client.request(proxied_request).await */
        println!("Allowing Request from IP {} to {}", addr, backend_server);
        
        // Construct the new URI
        let uri_string = format!(
            "http://{}{}",
            backend_server.authority().unwrap(),
            req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
        );
        let uri = uri_string.parse::<hyper::Uri>().unwrap();

        // Set the request URI
        *req.uri_mut() = uri;

        // Forward the request and return the response
        client.request(req).await
    } else {
        let mut response = Response::new(Body::from("Too many requests"));
        *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
        Ok(response)
    }
}
