#![recursion_limit = "512"]

use actix_web::{error::ErrorUnauthorized, guard::{Guard, GuardContext}, post, web::{self, Bytes}, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use awc::{error::{PayloadError, SendRequestError}, http::{uri::{InvalidUri, InvalidUriParts, Scheme}, Method, Uri}};
use base64::Engine;
use futures::StreamExt;
use lemmy_client::lemmy_api_common::{lemmy_db_schema::{source::{community::Community, post::Post}, CommentSortType, SortType}, lemmy_db_views::structs::PostView};
use log::{info, warn};
use serde::{Deserialize, Deserializer};

mod login;
mod session;
mod loid;
mod w3_reporting_policy;
mod v2c;

mod get_account;
mod get_ad_eligibility;
mod get_awards_for_sub;
mod get_badges;
mod get_blocked_users;
mod get_communities;
mod get_dynamic_configs;
mod get_home_feed;
mod get_inventory_items;
mod get_location;
mod get_marketing_nudges;
mod get_matrix_notifications;
mod get_preferences;
mod get_public_showcase;
mod get_subscribed_count;
mod get_username;
mod get_vaults;
mod register_mobile_push_token;
mod search_message_reactions;

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
        format!("t2_{}", self.id.0)
    }

    fn prefixed_name(&self) -> String {
        format!("u/{}", self.name)
    }

    fn path(&self) -> String {
        format!("/u/{}@{}", self.name, "todo") // TODO
    }
}

pub trait HackTraitCommunity {
    fn reddit_id(&self) -> String;
    fn prefixed_name(&self) -> String;
    fn link(&self) -> String;
}

impl HackTraitCommunity for Community {
    fn reddit_id(&self) -> String {
        format!("t5_{}", self.id.0)
    }

    fn prefixed_name(&self) -> String {
        format!("c/{}", self.name)
    }

    fn link(&self) -> String {
        format!("{}@{}", self.name, "todo") // TODO
    }
}

pub trait HackTraitPost {
    fn reddit_id(&self) -> String;
    fn reddit_id_base64(&self) -> String;
}

impl HackTraitPost for Post {
    fn reddit_id(&self) -> String {
        format!("t3_{}", self.id)
    }

    fn reddit_id_base64(&self) -> String {
        base64::prelude::BASE64_STANDARD.encode(self.reddit_id())
    }
}

pub trait HackTraitCommentSortType {
    fn to_reddit(&self) -> &'static str;
}

impl HackTraitCommentSortType for CommentSortType {
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

pub trait HackTraitSortType: Sized {
    fn deserialize_from_reddit<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
}

impl HackTraitSortType for Option<SortType> {
    fn deserialize_from_reddit<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let sort_type = String::deserialize(deserializer)?;
        match sort_type.as_str() {
            "BEST" => Ok(Some(SortType::Active)),
            "NEW" => Ok(Some(SortType::New)),
            "HOT" => Ok(Some(SortType::Hot)),
            "TOP" => Ok(Some(SortType::TopDay)),
            "CONTROVERSIAL" => Ok(Some(SortType::Controversial)),
            "RISING" => Ok(Some(SortType::Scaled)),
            "NONE" => Ok(None),
            _ => Err(serde::de::Error::custom(format!("Unknown sort type: {sort_type}"))),
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
        _ if body.is_empty() => format!("{target_request:?}\nno content"),
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
        _ if response_body.is_empty() => format!("{response:?}\nno content"),
        true => format!("{response:?}\n{}", String::from_utf8_lossy(&response_body)),
        false => format!("{response:?}\nbinary data"),
    };

    if target_domain != "matrix.redditspace.com" {
        println!("{log_part1}\n{log_part2}");
    }

    Ok(response)
}

struct Apollo(&'static str);

impl Guard for Apollo {
    fn check(&self, req: &GuardContext) -> bool {
        req.head().headers().get("x-apollo-operation-name").map(|o| o == self.0).unwrap_or(false)
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port = std::env::var("PORT").map(|p| p.parse().expect("Port must be a number")).unwrap_or(3000);

    let fut = HttpServer::new(|| {
        App::new()
            .service(login::login)
            .service(session::session)
            .service(w3_reporting_policy::w3_reporting_policy)
            .service(loid::loid)
            .service(v2c::v2c)
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("AdEligibilityForUser")).to(get_ad_eligibility::get_ad_eligibility))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("AllDynamicConfigs")).to(get_dynamic_configs::get_dynamic_configs))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("BadgeCount")).to(get_badges::get_badges))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("BlockedRedditors")).to(get_blocked_users::get_blocked_users))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetAccount")).to(get_account::get_account))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetAccountPreferences")).to(get_preferences::get_preferences))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetAllVaults")).to(get_vaults::get_vaults))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetAwardsForSubreddit")).to(get_awards_for_sub::get_awards_for_sub))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetInventoryItemsByIds")).to(get_inventory_items::get_inventory_items))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetPublicShowcaseOfCurrentUser")).to(get_public_showcase::get_public_showcase))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetRealUsername")).to(get_username::get_username))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("HomeFeedSdui")).to(get_home_feed::get_home_feed))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("IdentityMatrixNotifications")).to(get_matrix_notifications::get_matrix_notifications))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("MarketingNudges")).to(get_marketing_nudges::get_marketing_nudges))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("RegisterMobilePushToken")).to(register_mobile_push_token::register_mobile_push_token))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("SearchChatMessageReactionIcons")).to(search_message_reactions::search_message_reactions))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("SubscribedSubredditsCount")).to(get_subscribed_count::get_subscribed_count))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("UserLocation")).to(get_location::get_location))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("UsernameAndExperiments")).to(get_username::get_username))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("UserSubredditListItems")).to(get_communities::get_communities))
            .route("/gql-fed.reddit.com/", web::to(|req: HttpRequest| async move {
                    let operation_name = req.headers().get("x-apollo-operation-name").map(|o| o.to_str().unwrap()).unwrap_or("unknown");
                    warn!("Unknown Apollo operation: {operation_name}");
                    HttpResponse::Forbidden().finish()
            }))
            .default_service(web::route().to(proxy))
    })
    .bind(("127.0.0.1", port))?
    .run();

    info!("Server running at http://localhost:{port}");

    fut.await
}
