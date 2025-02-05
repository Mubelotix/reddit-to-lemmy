// {"operationName":"BadgeCount","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"6e5b40ea4193a6fcfd6890518f4cdde524e434243d055c33a552af2e42e0a433"}}}
//
// {"data":{"badgeIndicators":{"__typename":"BadgeIndicators","directMessages":{"count":0,"style":"NUMBERED"},"chatTab":{"count":0,"style":"NUMBERED"},"messageTab":{"count":0,"style":"NUMBERED"},"activityTab":{"count":0,"style":"NUMBERED"},"inboxTab":{"count":0,"style":"NUMBERED"},"appBadge":{"count":0,"style":"NUMBERED"},"chatHasNewMessages":{"style":"FILLED","isShowing":false}}}}

use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, LemmyRequest};
use serde_json::json;
use GetBadgesError::*;
use log::{debug, trace};

use crate::get_lemmy_client;

#[derive(Debug)]
pub enum GetBadgesError {
    Authentication,
    UnreadCount(LemmyErrorType),
}

impl std::fmt::Display for GetBadgesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            UnreadCount(e) => write!(f, "Unread count error: {}", e),
        }
    }
}

impl ResponseError for GetBadgesError {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("GetBadgesError: {self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_badges(request: HttpRequest) -> Result<HttpResponse, GetBadgesError> {
    debug!("get_badges");

    let (jwt, client) = get_lemmy_client(&request).ok_or(Authentication)?;

    let unread_count = client.unread_count(LemmyRequest { body: (), jwt: Some(jwt) }).await.map_err(UnreadCount)?;

    let rep = json! {{
        "data": {
            "badgeIndicators": {
                "__typename": "BadgeIndicators",
                "directMessages": {
                    "count": unread_count.private_messages,
                    "style": "NUMBERED"
                },
                "chatTab": {
                    "count": 0,
                    "style": "NUMBERED"
                },
                "messageTab": {
                    "count": unread_count.private_messages,
                    "style": "NUMBERED"
                },
                "activityTab": {
                    "count": unread_count.replies + unread_count.mentions,
                    "style": "NUMBERED"
                },
                "inboxTab": {
                    "count": unread_count.replies + unread_count.mentions, // TODO: Include applications
                    "style": "NUMBERED"
                },
                "appBadge": {
                    "count": 0,
                    "style": "NUMBERED"
                },
                "chatHasNewMessages": {
                    "style": "FILLED",
                    "isShowing": false
                }
            }
        }
    }};

    trace!("get_badges response: {}", serde_json::to_string(&rep).unwrap_or_default());
    Ok(HttpResponse::Ok().json(rep))
}
