use actix_web::{web::{self, Bytes}, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use awc::{error::{PayloadError, SendRequestError}, http::{uri::{InvalidUri, InvalidUriParts, Scheme}, Method, Uri}};
use futures::StreamExt;

const READABLE_BODIES: &[&str] = &[
    "application/x-www-form-urlencoded",
    "application/json",
    "multipart/form-data",
    "text/plain",
    "text/html",
    "text/css",
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
        println!("error");
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

async fn proxy(request: HttpRequest, mut payload: web::Payload) -> Result<impl Responder, ProxyError> {
    use ProxyError::*;
    
    let mut body = Vec::new();
    while let Some(item) = payload.next().await {
        body.extend_from_slice(&item.map_err(RequestPayload)?);
    }

    let readable_body = request.headers().get("content-type").map(|ct| ct.to_str().unwrap()).map(|ct| {
        READABLE_BODIES.iter().any(|rb| ct.starts_with(rb))
    }).unwrap_or(false);

    let uri = request.uri();
    let path = uri.path().trim_start_matches('/');
    let (target_domain, target_path) = path.split_once('/').unwrap_or((path, ""));
    let target_path = format!("/{}", target_path);

    let mut uri_parts = uri.clone().into_parts();
    uri_parts.scheme = Some(Scheme::HTTPS);
    uri_parts.authority = Some(target_domain.parse().map_err(InvalidAuthority)?);
    uri_parts.path_and_query = Some(target_path.parse().map_err(InvalidQuery)?);
    let target_uri = Uri::from_parts(uri_parts).map_err(InvalidUriParts)?;

    let client = awc::Client::default();
    let mut target_request = client.request_from(target_uri, request.head());
    target_request.headers_mut().remove("host");
    target_request.headers_mut().remove("x-forwarded-for");
    target_request.headers_mut().remove("x-forwarded-proto");
    target_request.headers_mut().remove("x-forwarded-protocol");
    target_request.headers_mut().remove("x-forwarded-port");
    target_request.headers_mut().remove("x-forwarded-host");
    target_request.headers_mut().remove("x-real-ip");
    target_request.headers_mut().remove("cookie");
    target_request.headers_mut().remove("connection");
    
    let log = match readable_body {
        true => format!("{target_request:?}\n{:?}", String::from_utf8_lossy(&body)),
        false => format!("{target_request:?}\nbinary"),
    };

    let mut target_response = match request.method() == Method::GET || body.is_empty() {
        true => target_request.send().await.map_err(SendRequest)?,
        false => target_request.send_body(body).await.map_err(SendRequest)?,
    };
    let response_body = target_response.body().await.map_err(ResponsePayload)?;


    if target_response.status().is_client_error() || target_response.status().is_server_error() {
        println!("{log}");
        
        let readable_body = target_response.headers().get("content-type").map(|ct| ct.to_str().unwrap()).map(|ct| {
            READABLE_BODIES.iter().any(|rb| ct.starts_with(rb))
        }).unwrap_or(false);
    
        match readable_body {
            true => println!("{target_response:?}\n{:?}", String::from_utf8_lossy(&response_body)),
            false => println!("{target_response:?}\nbinary data"),
        }
    }

    let mut response_builder = HttpResponse::new(target_response.status());
    *response_builder.headers_mut() = target_response.headers().clone();
    response_builder.headers_mut().append("content-length".try_into().unwrap(), response_body.len().to_string().parse().unwrap());
    response_builder.headers_mut().remove("transfer-encoding");
    response_builder.headers_mut().remove("content-encoding");
    let response = response_builder.set_body(response_body);

    println!("{response:?}");

    Ok(response)
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
