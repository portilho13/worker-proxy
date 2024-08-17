use std::net::IpAddr;
use std::str::FromStr;

use hyper::header::{HeaderName, HeaderValue};
use hyper::{Body, Client, HeaderMap, Request, Response, StatusCode};
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
/*         let uri_string = format!(
            "http://{}{}",
            backend_server.authority().unwrap(),
            req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
        );
        let uri = uri_string.parse::<hyper::Uri>().unwrap();

        // Set the request URI
        *req.uri_mut() = uri;

        println!("Request Headers: {:?}", req.headers());

        // Forward the request and return the response
        client.request(req).await */

        println!("Request Headers: {:?}", req.headers());

        let headers: &HeaderMap<HeaderValue> = req.headers();


        let mut request_builder = Request::builder()
            .method(req.method().clone())
            .uri(backend_server.clone());

        for (name, value) in headers.iter() {
            if name != "Connection" && name != "Host" {
                request_builder = request_builder.header(name, value);
            }
        }

        let host_header_name: &HeaderName = &HeaderName::from_str("Host").unwrap();

        let host_header_value: &HeaderValue = &HeaderValue::from_str(&backend_server.to_string()).unwrap();

        request_builder = request_builder.header(host_header_name, host_header_value);


        let proxied_request = request_builder
            .body(req.into_body())
            .expect("Failed to build request");
    
        client.request(proxied_request).await
    } else {
        let mut response = Response::new(Body::from("Too many requests"));
        *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
        Ok(response)
    }
}
