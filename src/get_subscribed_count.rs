// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-session": "redacted"
//     "accept-encoding": "gzip"
//     "content-type": "application/json; charset=utf-8"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-loid": "redacted"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "x-reddit-compression": "1"
//     "accept-language": "en-US,en;q=0.9"
//     "x-apollo-operation-id": "b1bbea66ca4ae9e4bbe444aff2a09bc3a1e4309c06df971c84f98e65b66baaec"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "x-reddit-width": "411"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "content-length": "198"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-dpr": "2.625"
//     "authorization": "Bearer redacted"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-apollo-operation-name": "SubscribedSubredditsCount"

// {"operationName":"SubscribedSubredditsCount","variables":{"first":5000},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"b1bbea66ca4ae9e4bbe444aff2a09bc3a1e4309c06df971c84f98e65b66baaec"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "content-length": "5566"
//     "x-ratelimit-remaining": "1986.0"
//     "via": "1.1 varnish"
//     "accept-ranges": "bytes"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "x-content-type-options": "nosniff"
//     "x-ratelimit-reset": "85"
//     "content-type": "application/json"
//     "x-reddit-session": "redacted"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//     "date": "Tue, 28 Jan 2025 17:18:34 GMT"
//     "vary": "origin"
//     "x-frame-options": "SAMEORIGIN"
//     "x-ratelimit-used": "14"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "apollo-trace-id": "9dc358be2d2b951a068a7920e44babad"
//     "server": "snooserv"
//     "x-xss-protection": "1; mode=block"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//   body: Sized(5566)
//  }
// {"data":{"identity":{"subscribedSubreddits":{"edges":[{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}},{"node":{"__typename":"Subreddit"}}]}}}}

use std::cmp::max;
use actix_web::{web::Json, HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, ClientOptions, LemmyClient, LemmyRequest};
use serde::Deserialize;
use serde_json::json;
use crate::{get_jwt, GraphQlRequest};
use log::{debug, trace};
use GetSubscribedCountError::*;

#[derive(Debug)]
pub enum GetSubscribedCountError {
    Authentication,
    GetSite(LemmyErrorType),
    MissingUser,
}

impl std::fmt::Display for GetSubscribedCountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            GetSite(e) => write!(f, "Error getting site: {e}"),
            MissingUser => write!(f, "Missing user"),
        }
    }
}

impl ResponseError for GetSubscribedCountError {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("GetSubscribedCountError: {self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubscribedCountVariables {
    first: usize
}

pub async fn get_subscribed_count(request: HttpRequest, body: Json<GraphQlRequest<GetSubscribedCountVariables>>) -> Result<HttpResponse, GetSubscribedCountError> {
    debug!("get_subscribed_count");

    let jwt = get_jwt(&request).ok_or(Authentication)?;

    let client = LemmyClient::new(ClientOptions {
        domain: String::from("jlai.lu"),
        secure: true
    });

    let posts = client.get_site(LemmyRequest { body: (), jwt: Some(jwt.clone()) }).await.map_err(GetSite)?;
    let my_user = posts.my_user.ok_or(MissingUser)?;
    let count = max(my_user.follows.len(), body.variables.first);

    let rep = json! {{
        "data": {
            "identity": {
                "subscribedSubreddits": {
                    "edges": (0..count).map(|_community| json! {{
                        "node": {
                            "__typename": "Subreddit",
                        }
                    }}).collect::<Vec<_>>()
                }
            }
        }
    }};
    
    trace!("get_subscribed_count response: {rep}");
    Ok(HttpResponse::Ok().json(rep))
}

