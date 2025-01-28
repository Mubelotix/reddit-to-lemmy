#![recursion_limit = "512"]

use actix_web::{guard::{Guard, GuardContext}, post, web::{self, Bytes}, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use awc::{error::{PayloadError, SendRequestError}, http::{uri::{InvalidUri, InvalidUriParts, Scheme}, Method, Uri}};
use futures::StreamExt;
use lemmy_client::lemmy_api_common::lemmy_db_schema::{CommentSortType, SortType};
use serde::Deserialize;

mod login;
mod session;
mod get_account;
mod get_ad_eligibility;
mod get_badges;
mod get_blocked_users;
mod get_inventory_items;
mod get_marketing_nudges;
mod get_matrix_notifications;
mod get_preferences;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlRequest<V> {
    operation_name: String,
    variables: V,
    extensions: serde_json::Value,
}

pub trait HackTraitPerson {
    fn reddit_id(&self) -> String;
    fn prefixed_name(&self) -> String;
    fn path(&self) -> String;
}

impl HackTraitPerson for lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person {
    fn reddit_id(&self) -> String {
        format!("t2_{}_{}", self.id.0, self.instance_id.0)
    }

    fn prefixed_name(&self) -> String {
        format!("u/{}", self.name)
    }

    fn path(&self) -> String {
        format!("/u/{}@{}", self.name, "todo") // TODO
    }
}

pub trait HackTraitSortType {
    fn to_reddit(&self) -> &'static str;
}

impl HackTraitSortType for CommentSortType {
    fn to_reddit(&self) -> &'static str {
        match self {
            CommentSortType::Hot => "CONFIDENCE",
            CommentSortType::Top => "TOP",
            CommentSortType::New => "NEW",
            CommentSortType::Old => "OLD",
            CommentSortType::Controversial => "CONTROVERSIAL",
        }
    }
}

pub fn get_jwt(req: &HttpRequest) -> Option<String> {
    let autorization = req.headers().get("authorization")?.to_str().ok()?;
    let jwt = autorization.split_once(' ')?.1.to_owned();
    Some(jwt)
}

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
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("{self}");

        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn proxy(request: HttpRequest, mut payload: web::Payload) -> Result<impl Responder, ProxyError> {
    use ProxyError::*;
    
    let mut body = Vec::new();
    while let Some(item) = payload.next().await {
        body.extend_from_slice(&item.map_err(RequestPayload)?);
    }

    let request_body_readable = request.headers().get("content-type").map(|ct| ct.to_str().unwrap()).map(|ct| {
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
    target_request.headers_mut().remove("connection");
    
    let log_part1 = match request_body_readable {
        true => format!("{target_request:?}\n{}", String::from_utf8_lossy(&body)),
        false => format!("{target_request:?}\nbinary"),
    };

    let mut target_response = match request.method() == Method::GET || body.is_empty() {
        true => target_request.send().await.map_err(SendRequest)?,
        false => target_request.send_body(body).await.map_err(SendRequest)?,
    };
    let response_body = target_response.body().await.map_err(ResponsePayload)?;

    let response_body_readable = target_response.headers().get("content-type").map(|ct| ct.to_str().unwrap()).map(|ct| {
        READABLE_BODIES.iter().any(|rb| ct.starts_with(rb))
    }).unwrap_or(false);    

    let mut response_builder = HttpResponse::new(target_response.status());
    *response_builder.headers_mut() = target_response.headers().clone();
    response_builder.headers_mut().insert("content-length".try_into().unwrap(), response_body.len().to_string().parse().unwrap());
    response_builder.headers_mut().remove("transfer-encoding");
    response_builder.headers_mut().remove("content-encoding");
    // response_builder.headers_mut().remove("x-reddit-loid");
    // if let Some(old_cookies) = response_builder.headers_mut().remove("set-cookie").map(|c| c.to_str().unwrap().to_string()).next() {
    //     let new_cookies = old_cookies.replace("reddit.com;", "3000.code.mub.lol;");
    //     response_builder.headers_mut().insert("set-cookie".try_into().unwrap(), new_cookies.parse().unwrap());
    // }
    
    let response = response_builder.set_body(response_body.clone());

    let log_part2 = match response_body_readable {
        true => format!("{response:?}\n{}", String::from_utf8_lossy(&response_body)),
        false => format!("{response:?}\nbinary data"),
    };

    if target_domain != "matrix.redditspace.com" {
        println!("{log_part1}\n{log_part2}");
    }

    Ok(response)
}

struct ApolloOperation(&'static str);

impl Guard for ApolloOperation {
    fn check(&self, req: &GuardContext) -> bool {
        req.head().headers().get("x-apollo-operation-name").map(|o| o == self.0).unwrap_or(false)
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").map(|p| p.parse().expect("Port must be a number")).unwrap_or(3000);

    let fut = HttpServer::new(|| {
        App::new()
            .service(login::login)
            .service(session::session)
            .route("/gql-fed.reddit.com", web::post()
                .guard(ApolloOperation("AdEligibilityForUser")).to(get_ad_eligibility::get_ad_eligibility)
                .guard(ApolloOperation("BadgeCount")).to(get_badges::get_badges)
                .guard(ApolloOperation("BlockedRedditors")).to(get_blocked_users::get_blocked_users)
                .guard(ApolloOperation("GetAccount")).to(get_account::get_account)
                .guard(ApolloOperation("GetAccountPreferences")).to(get_preferences::get_preferences)
                .guard(ApolloOperation("GetInventoryItemsByIds")).to(get_inventory_items::get_inventory_items)
                .guard(ApolloOperation("IdentityMatrixNotifications")).to(get_matrix_notifications::get_matrix_notifications)
                .guard(ApolloOperation("MarketingNudges")).to(get_marketing_nudges::get_marketing_nudges)
            )
            .default_service(web::route().to(proxy))
    })
    .bind(("127.0.0.1", port))?
    .run();

    println!("Server running at http://localhost:{port}");

    fut.await
}
