// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-compression": "1"
//     "x-apollo-operation-name": "RegisterMobilePushToken"
//     "x-apollo-operation-id": "35313b81bfbb621c426b2f2ec276db3db476a5d8362a1ddbeec982a01dc74977"
//     "content-type": "application/json; charset=utf-8"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "accept-encoding": "gzip"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "content-length": "1846"
//     "x-reddit-width": "411"
//     "x-reddit-loid": "redacted"
//     "x-reddit-dpr": "2.625"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "accept-language": "en-US,en;q=0.9"
//     "authorization": "Bearer redacted"
// 
// {"operationName":"RegisterMobilePushToken","variables":{"authTokens":["invalid-token","redacted"],"pushToken":"redacted","deviceId":"6918b40d-1a50-4478-8b05-ad96845c1ae3","timezoneName":"Europe/Paris","timestamp":"2025-01-28T17:18:29.615","language":"en-US"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"35313b81bfbb621c426b2f2ec276db3db476a5d8362a1ddbeec982a01dc74977"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "x-ratelimit-reset": "86"
//     "via": "1.1 varnish"
//     "content-type": "application/json"
//     "apollo-trace-id": "b0b0fe649d292de76a812e1448fe4ad8"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "vary": "origin"
//     "x-xss-protection": "1; mode=block"
//     "accept-ranges": "bytes"
//     "content-length": "48"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//     "server": "snooserv"
//     "x-reddit-session": "redacted"
//     "x-ratelimit-used": "4"
//     "date": "Tue, 28 Jan 2025 17:18:33 GMT"
//     "x-ratelimit-remaining": "1996.0"
//     "x-content-type-options": "nosniff"
//     "x-frame-options": "SAMEORIGIN"
//   body: Sized(48)
//  }
// {"data":{"registerMobilePushToken":{"ok":true}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn register_mobile_push_token() -> HttpResponse {
    debug!("register_mobile_push_token");

    // TODO: It would be wonderful to have a real implementation here.

    let rep = json! {{
        "data": {
            "registerMobilePushToken": {
                "ok": true
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
