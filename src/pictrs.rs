/// ALlow any request like GET:/lemmy.world/pictrs/image/cab199bb-059b-4a18-b624-ac1ed6b7160a.jpeg
/// Capture like that <domain>/pictrs/image/<path>

use actix_web::{get, web, HttpResponse, ResponseError};
use PictrsError::*;

#[derive(Debug)]
enum PictrsError {
    CannotForwardRequest(awc::error::SendRequestError),
    PayloadError(actix_web::error::PayloadError),
}

impl std::fmt::Display for PictrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CannotForwardRequest(e) => write!(f, "Cannot forward request: {e}"),
            PayloadError(e) => write!(f, "Payload error: {e}"),
        }
    }
}

impl ResponseError for PictrsError {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("PictrsError: {self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[get("/{domain}/pictrs/image/{path}")]
async fn pictrs(path: web::Path<(String,String)>) -> Result<HttpResponse, PictrsError> {
    let (domain, path) = path.into_inner();

    let target_uri = format!("https://{}/pictrs/image/{}", domain, path);

    let client = awc::Client::default();
    let target_request = client.get(target_uri);
    let mut target_response = target_request.send().await.map_err(CannotForwardRequest)?;
    let response_body = target_response.body().await.map_err(PayloadError)?;
    let content_type = target_response.headers().get("content-type").and_then(|ct| ct.to_str().ok()).unwrap_or("application/octet-stream");

    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .body(response_body))
}
