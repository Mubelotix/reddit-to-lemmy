// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "accept-encoding": "gzip"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-apollo-operation-id": "95192e490742fec36721e64002e1913e6f0dfc746b6033f31c37f8b8689595f6"
//     "x-reddit-dpr": "2.625"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-apollo-operation-name": "IdentityMatrixNotifications"
//     "authorization": "Bearer redacted"
//     "x-reddit-compression": "1"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "accept-language": "en-US,en;q=0.9"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "content-length": "188"
//     "x-reddit-session": "redacted"
//     "x-reddit-width": "411"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "content-type": "application/json; charset=utf-8"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-reddit-loid": "redacted"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//
// {"operationName":"IdentityMatrixNotifications","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"95192e490742fec36721e64002e1913e6f0dfc746b6033f31c37f8b8689595f6"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "x-xss-protection": "1; mode=block"
//     "x-ratelimit-used": "5"
//     "via": "1.1 varnish"
//     "vary": "origin"
//     "x-ratelimit-remaining": "1995.0"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "x-frame-options": "SAMEORIGIN"
//     "content-length": "66"
//     "apollo-trace-id": "209752cf83561f2b906a189c1d9c7080"
//     "x-content-type-options": "nosniff"
//     "date": "Tue, 28 Jan 2025 17:18:33 GMT"
//     "x-ratelimit-reset": "86"
//     "content-type": "application/json"
//     "accept-ranges": "bytes"
//     "server": "snooserv"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//   body: Sized(66)
//  }
// {"data":{"identity":{"matrixNotifications":{"unreadCount":null}}}}

use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, ClientOptions, LemmyClient, LemmyRequest};
use serde_json::json;
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

    Ok(HttpResponse::Ok().json(rep))
}

