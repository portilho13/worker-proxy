use hyper::{Body, Client, Request, Response};
use hyper::client::HttpConnector;

pub async fn handle_proxy_request(
    req: Request<Body>,
    client: Client<HttpConnector>,
    backend_server: hyper::Uri,
) -> Result<Response<Body>, hyper::Error> {
    // Construct a new request based on the incoming request
    let proxied_request = Request::builder()
        .method(req.method().clone())
        .uri(backend_server.clone())
        .body(req.into_body())
        .expect("Failed to build request");

    // Send the request to the backend server
    client.request(proxied_request).await
}
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         