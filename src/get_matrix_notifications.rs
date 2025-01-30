// {"operationName":"IdentityMatrixNotifications","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"95192e490742fec36721e64002e1913e6f0dfc746b6033f31c37f8b8689595f6"}}}
// 
// {"data":{"identity":{"matrixNotifications":{"unreadCount":null}}}}

use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, ClientOptions, LemmyClient, LemmyRequest};
use serde_json::json;
use log::{debug, trace};
use GetMatrixNotificationsError::*;

use crate::get_jwt;

#[derive(Debug)]
pub enum GetMatrixNotificationsError {
    Authentication,
    UnreadCount(LemmyErrorType),
}

impl std::fmt::Display for GetMatrixNotificationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            UnreadCount(e) => write!(f, "Unread count error: {}", e),
        }
    }
}

impl ResponseError for GetMatrixNotificationsError {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("GetMatrixNotificationsError: {self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_matrix_notifications(request: HttpRequest) -> Result<HttpResponse, GetMatrixNotificationsError> {
    debug!("get_matrix_notifications");

    let jwt = get_jwt(&request).ok_or(Authentication)?;

    let client = LemmyClient::new(ClientOptions {
        domain: String::from("jlai.lu"),
        secure: true
    });

    let unread_count = client.unread_count(LemmyRequest { body: (), jwt: Some(jwt) }).await.map_err(UnreadCount)?;

    let rep = json! {{
        "data": {
            "identity": {
                "matrixNotifications": {
                    "unreadCount": unread_count // TODO: Check behavior when there are actual unread messages
                }
            }
        }
    }};

    trace!("get_matrix_notifications response: {rep}");
    Ok(HttpResponse::Ok().json(rep))
}

