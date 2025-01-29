// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "content-length": "176"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-apollo-operation-name": "GetRealUsername"
//     "content-type": "application/json; charset=utf-8"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "accept-encoding": "gzip"
//     "x-apollo-operation-id": "2dc95e27aa590b6b5ef6eac8f074b92c09fefd4c1de0f1d8af91de865a542e38"
//     "x-reddit-qos": "down-rate-mbps=127.269"
//     "authorization": "Bearer redacted"
//     "x-reddit-compression": "1"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"

// {"operationName":"GetRealUsername","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"2dc95e27aa590b6b5ef6eac8f074b92c09fefd4c1de0f1d8af91de865a542e38"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "content-type": "application/json"
//     "x-frame-options": "SAMEORIGIN"
//     "via": "1.1 varnish"
//     "date": "Tue, 28 Jan 2025 16:48:19 GMT"
//     "apollo-trace-id": "3fd16f42e5b9130545adeee7568b239b"
//     "x-xss-protection": "1; mode=block"
//     "x-ratelimit-remaining": "1999.0"
//     "accept-ranges": "bytes"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "content-length": "55"
//     "x-ratelimit-reset": "100"
//     "server": "snooserv"
//     "vary": "origin"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "x-content-type-options": "nosniff"
//     "x-ratelimit-used": "1"
//   body: Sized(55)
//  }
// {"data":{"identity":{"redditor":{"name":"Mubelotix"}}}}


use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, ClientOptions, LemmyClient, LemmyRequest};
use serde_json::json;
use crate::get_jwt;
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

    let jwt = get_jwt(&request).ok_or(Authentication)?;

    let client = LemmyClient::new(ClientOptions {
        domain: String::from("jlai.lu"),
        secure: true
    });

    debug!("{jwt}");

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
    
    trace!("get_username response: {rep}");
    Ok(HttpResponse::Ok().json(rep))
}
