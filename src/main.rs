use actix_web::{web::{self, Bytes}, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use awc::{error::{PayloadError, SendRequestError}, http::{uri::{InvalidUri, InvalidUriParts, Scheme}, Uri}};
use futures::StreamExt;

const READABLE_BODIES: &[&str] = &[
    "application/x-www-form-urlencoded",
    "application/json",
    "multipart/form-data",
    "text/plain",
];

#[derive(Debug)]
enum ProxyError {
    RequestPayload(PayloadError),
    InvalidAuthority(InvalidUri),
    InvalidQuery(InvalidUri),
    InvalidUriParts(InvalidUriParts),
    SendRequest(SendRequestError),
    ResponsePayload(PayloadError),
}

impl std::fmt::Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProxyError::RequestPayload(e) => write!(f, "Request payload error: {}", e),
            ProxyError::InvalidAuthority(e) => write!(f, "Invalid authority: {}", e),
            ProxyError::InvalidQuery(e) => write!(f, "Invalid query: {}", e),
            ProxyError::InvalidUriParts(e) => write!(f, "Invalid URI parts: {}", e),
            ProxyError::SendRequest(e) => write!(f, "Send request error: {}", e),
            ProxyError::ResponsePayload(e) => write!(f, "Response payload error: {}", e),
        }
    }
}

impl ResponseError for ProxyError {
}

async fn proxy(mut request: HttpRequest) -> Result<impl Responder, ProxyError> {
    use ProxyError::*;

    let body = match request.take_payload() {
        actix_web::dev::Payload::None => Vec::new(),
        actix_web::dev::Payload::Stream { payload: () } => Vec::new(),
        actix_web::dev::Payload::H1 { mut payload } => {
            let mut body = Vec::new();
            while let Some(chunk) = payload.next().await {
                body.extend_from_slice(&chunk.map_err(RequestPayload)?);
            }
            body
        },
        actix_web::dev::Payload::H2 { mut payload } => {
            let mut body = Vec::new();
            while let Some(chunk) = payload.next().await {
                body.extend_from_slice(&chunk.map_err(RequestPayload)?);
            }
            body
        },
    };

    let readable_body = request.headers().get("content-type").map(|ct| ct.to_str().unwrap()).map(|ct| {
        READABLE_BODIES.iter().any(|rb| ct.starts_with(rb))
    }).unwrap_or(false);

    match readable_body {
        true => println!("{request:?}\n{:?}", String::from_utf8_lossy(&body)),
        false => println!("{request:?}\n{body:?}"),
    }

    let uri = request.uri();
    let path = uri.path().trim_start_matches('/');
    let (target_domain, target_path) = path.split_once('/').unwrap_or((path, ""));

    let mut uri_parts = uri.clone().into_parts();
    uri_parts.scheme = Some(Scheme::HTTPS);
    uri_parts.authority = Some(target_domain.parse().map_err(InvalidAuthority)?);
    uri_parts.path_and_query = Some(target_path.parse().map_err(InvalidQuery)?);
    let target_uri = Uri::from_parts(uri_parts).map_err(InvalidUriParts)?;

    let client = awc::Client::default();
    let mut target_request = client.request_from(target_uri, request.head());
    target_request.headers_mut().remove("host");
    let mut target_response = target_request.send_body(body).await.map_err(SendRequest)?;
    let response_body = target_response.body().await.map_err(ResponsePayload)?;

    let readable_body = target_response.headers().get("content-type").map(|ct| ct.to_str().unwrap()).map(|ct| {
        READABLE_BODIES.iter().any(|rb| ct.starts_with(rb))
    }).unwrap_or(false);

    match readable_body {
        true => println!("{target_response:?}\n{:?}", String::from_utf8_lossy(&response_body)),
        false => println!("{target_response:?}\n{response_body:?}"),
    }

    let mut response_builder = HttpResponse::new(target_response.status());
    *response_builder.headers_mut() = target_response.headers().clone();
    Ok(response_builder.set_body(response_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").map(|p| p.parse().expect("Port must be a number")).unwrap_or(3000);

    let fut = HttpServer::new(|| {
        App::new()
            .default_service(web::route().to(proxy))
    })
    .bind(("127.0.0.1", port))?
    .run();

    println!("Server running at http://localhost:{port}");

    fut.await
}
