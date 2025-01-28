// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "accept-language": "en-US,en;q=0.9"
//     "x-reddit-compression": "1"
//     "x-reddit-session": "redacted"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "x-reddit-width": "411"
//     "x-apollo-operation-id": "6e5b40ea4193a6fcfd6890518f4cdde524e434243d055c33a552af2e42e0a433"
//     "x-reddit-loid": "redacted"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "content-length": "171"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-apollo-operation-name": "BadgeCount"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "authorization": "Bearer redacted"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-dpr": "2.625"
//     "content-type": "application/json; charset=utf-8"
//     "accept-encoding": "gzip"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//
// {"operationName":"BadgeCount","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"6e5b40ea4193a6fcfd6890518f4cdde524e434243d055c33a552af2e42e0a433"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "x-ratelimit-used": "6"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "content-type": "application/json"
//     "via": "1.1 varnish"
//     "server": "snooserv"
//     "x-reddit-session": "redacted"
//     "vary": "origin"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//     "content-length": "381"
//     "x-frame-options": "SAMEORIGIN"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "apollo-trace-id": "6b143b62ab024fb009f9e26454619026"
//     "accept-ranges": "bytes"
//     "x-ratelimit-remaining": "1994.0"
//     "x-content-type-options": "nosniff"
//     "date": "Tue, 28 Jan 2025 17:18:34 GMT"
//     "x-xss-protection": "1; mode=block"
//     "x-ratelimit-reset": "86"
//   body: Sized(381)
//  }
// {"data":{"badgeIndicators":{"__typename":"BadgeIndicators","directMessages":{"count":0,"style":"NUMBERED"},"chatTab":{"count":0,"style":"NUMBERED"},"messageTab":{"count":0,"style":"NUMBERED"},"activityTab":{"count":0,"style":"NUMBERED"},"inboxTab":{"count":0,"style":"NUMBERED"},"appBadge":{"count":0,"style":"NUMBERED"},"chatHasNewMessages":{"style":"FILLED","isShowing":false}}}}

use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, ClientOptions, LemmyClient, LemmyRequest};
use serde_json::json;
use GetBadges::*;

use crate::get_jwt;

#[derive(Debug)]
pub enum GetBadges {
    Authentication,
    UnreadCount(LemmyErrorType),
}

impl std::fmt::Display for GetBadges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            UnreadCount(e) => write!(f, "Unread count error: {}", e),
        }
    }
}

impl ResponseError for GetBadges {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("{self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_badges(request: HttpRequest) -> Result<HttpResponse, GetBadges> {
    let jwt = get_jwt(&request).ok_or(Authentication)?;

    let client = LemmyClient::new(ClientOptions {
        domain: String::from("jlai.lu"),
        secure: true
    });

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

    Ok(HttpResponse::Ok().json(rep))
}
