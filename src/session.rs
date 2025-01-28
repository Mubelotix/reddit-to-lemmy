use actix_web::{post, HttpRequest, HttpResponseBuilder, Responder};
use serde_json::json;
use awc::{cookie::CookieJar, http::StatusCode};



// ClientRequest HTTP/1.1 POST https://www.reddit.com/auth/v2/oauth/access-token/session
//   headers:
//     "authorization": "Basic b2hYcG9xclpZdWIxa2c6"
//     "x-reddit-compression": "1"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-qos": "down-rate-mbps=1.700"
//     "content-type": "application/json; charset=UTF-8"
//     "content-length": "30"
//     "accept-encoding": "gzip"
//     "cookie": "reddit_session=redacted jwt token (the one we gave in the login response)"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-attestation-device-token": "redacted jwt token"
// {"scopes":["*","email","pii"]}
//
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "content-type": "application/json"
//     "vary": "Accept-Encoding"
//     "x-frame-options": "SAMEORIGIN"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 1.0, \"failure_fraction\": 1.0}"
//     "x-reddit-session": "mcfdhqlrcefifajbfd.0.1738081120556.Z0FBQUFBQm5tUU5nVW4yZ1lIR2FRdWlFU2VJZnJhb3VVbldDem1saktKN1lnVDk0dEhBUG1FM21GSnUxdU8zQkk2Y3ZxZ1JManhNN2hTM0dHOHBoVkpCVWxGYW9YdXRtcXltbnFubkRlVVNpbGZUYVFrRFBFVmQtUV9KOTJyTEo3VFVEXzFQa0FCa2Y"
//     "accept-ranges": "bytes"
//     "date": "Tue, 28 Jan 2025 16:18:40 GMT"
//     "x-content-type-options": "nosniff"
//     "server": "snooserv"
//     "x-xss-protection": "1; mode=block"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "content-length": "1452"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "cache-control": "private, max-age=3600"
//     "set-cookie": "edgebucket=hok9NECvu9YPU027J7; Domain=reddit.com; Max-Age=63071999; Path=/;  secure"
//     "via": "1.1 varnish"
//   body: Sized(1452)
//  }
// {"access_token":"redacted jwt token", "expiry_ts":1738167520, "expires_in":86399, "scope":["*", "email", "pii"], "token_type":"bearer"}

#[post("/www.reddit.com/auth/v2/oauth/access-token/session")]
async fn session(request: HttpRequest) -> impl Responder {
    let cookie_header = request.headers().get("cookie").unwrap().to_str().unwrap();
    let jwt = cookie_header.split_once('=').unwrap().1;

    HttpResponseBuilder::new(StatusCode::OK)
        .json(json! {{
            "access_token": jwt,
            "expiry_ts": 1738167520,
            "expires_in": 86399,
            "scope": ["*", "email", "pii"],
            "token_type": "bearer"
        }})
}
