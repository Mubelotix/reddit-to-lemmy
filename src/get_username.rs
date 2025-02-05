// {"operationName":"GetRealUsername","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"2dc95e27aa590b6b5ef6eac8f074b92c09fefd4c1de0f1d8af91de865a542e38"}}}
// 
// {"data":{"identity":{"redditor":{"name":"Mubelotix"}}}}


use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, LemmyRequest};
use serde_json::json;
use crate::get_lemmy_client;
use log::{debug, trace};
use GetUsernameError::*;

#[derive(Debug)]
pub enum GetUsernameError {
    Authentication,
    GetSite(LemmyErrorType),
    MissingUser,
}

impl std::fmt::Display for GetUsernameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            GetSite(e) => write!(f, "GetSite error: {e}"),
            MissingUser => write!(f, "Missing user error"),
        }
    }
}

impl ResponseError for GetUsernameError {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("GetUsernameError: {self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_username(request: HttpRequest) -> Result<HttpResponse, GetUsernameError> {
    debug!("get_username");

    let (jwt, client) = get_lemmy_client(&request).ok_or(Authentication)?;

    let site = client.get_site(LemmyRequest { body: (), jwt: Some(jwt.clone()) }).await.map_err(GetSite)?;
    let my_user = site.my_user.ok_or(MissingUser)?;

    let rep = json! {{
        "data": {
            "identity": {
                "redditor": {
                    "name": my_user.local_user_view.person.name
                }
            }
        }
    }};
    
    trace!("get_username response: {}", serde_json::to_string(&rep).unwrap_or_default());
    Ok(HttpResponse::Ok().json(rep))
}
