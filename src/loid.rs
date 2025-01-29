// The loid procedure appears to be used by reddit to recover from a past session even if the account was manually disconnected and even if the data was wiped. We disable it by faking an internal server error.
// 
// ClientRequest HTTP/1.1 POST https://www.reddit.com/auth/v2/oauth/access-token/loid
//   headers:
//     "content-type": "application/json; charset=UTF-8"
//     "content-length": "30"
//     "x-reddit-compression": "1"
//     "x-attestation-device-token": "redacted"
//     "x-reddit-qos": "down-rate-mbps=127.269"
//     "x-reddit-loid": "redacted"
//     "authorization": "Basic redacted BUT THIS IS SHORT NOT JWT"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "accept-encoding": "gzip"
// 
// {"scopes":["*","email","pii"]}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 1.0, \"failure_fraction\": 1.0}"
//     "content-type": "application/json"
//     "vary": "Accept-Encoding"
//     "date": "Tue, 28 Jan 2025 16:48:19 GMT"
//     "x-xss-protection": "1; mode=block"
//     "set-cookie": "edgebucket=sYuIWoNYTbRyf1XM4l; Domain=reddit.com; Max-Age=63071999; Path=/;  secure"
//     "x-content-type-options": "nosniff"
//     "cache-control": "private, max-age=3600"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "via": "1.1 varnish"
//     "content-length": "1355"
//     "x-reddit-session": "redacted"
//     "accept-ranges": "bytes"
//     "server": "snooserv"
//     "x-frame-options": "SAMEORIGIN"
//   body: Sized(1355)
//  }
// {"access_token":"redacted", "expiry_ts":1738169299, "expires_in":86399, "scope":["*", "email", "pii"], "token_type":"bearer"}

use actix_web::{post, HttpResponse};
use log::debug;

#[post("www.reddit.com/auth/v2/oauth/access-token/loid")]
pub async fn loid() -> HttpResponse {
    debug!("loid");

    HttpResponse::InternalServerError().finish()
}

