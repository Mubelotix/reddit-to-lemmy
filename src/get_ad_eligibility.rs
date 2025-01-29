// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-apollo-operation-id": "39a5a68f81a20f9f7dafb72f017022d0ced17493d564e2fa8ff1286774eab6e6"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "content-length": "181"
//     "authorization": "Bearer redacted"
//     "x-reddit-width": "411"
//     "content-type": "application/json; charset=utf-8"
//     "x-reddit-compression": "1"
//     "x-apollo-operation-name": "AdEligibilityForUser"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "accept-encoding": "gzip"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "x-reddit-session": "redacted"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-dpr": "2.625"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "accept-language": "en-US,en;q=0.9"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-loid": "redacted"
// 
// {"operationName":"AdEligibilityForUser","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"39a5a68f81a20f9f7dafb72f017022d0ced17493d564e2fa8ff1286774eab6e6"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "server": "snooserv"
//     "content-length": "59"
//     "date": "Tue, 28 Jan 2025 17:18:34 GMT"
//     "accept-ranges": "bytes"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//     "x-ratelimit-reset": "85"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "x-content-type-options": "nosniff"
//     "apollo-trace-id": "35f6974cb71f02c9c01f74f33b12be72"
//     "via": "1.1 varnish"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "x-frame-options": "SAMEORIGIN"
//     "x-reddit-session": "redacted"
//     "x-ratelimit-used": "10"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "x-ratelimit-remaining": "1990.0"
//     "x-xss-protection": "1; mode=block"
//     "content-type": "application/json"
//     "vary": "origin"
//   body: Sized(59)
//  }
// {"data":{"adEligibility":{"userAdEligibility":"ELIGIBLE"}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_ad_eligibility() -> HttpResponse {
    debug!("get_ad_eligibility");

    let rep = json! {{
        "data": {
            "adEligibility": {
                "userAdEligibility": "NOT_ELIGIBLE_CONTEXT" // Value randomly found in the source code. Not sure what this means nor what it does.
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
