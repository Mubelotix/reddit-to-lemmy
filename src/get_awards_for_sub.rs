// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-qos": "down-rate-mbps=1.700"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-reddit-loid": "redacted"
//     "content-type": "application/json; charset=utf-8"
//     "x-reddit-dpr": "2.625"
//     "x-apollo-operation-name": "GetAwardsForSubreddit"
//     "x-apollo-operation-id": "80f71cc9610f3ff57310d00b45707789c8ddfd9dd86c4ea9f626b26fbf7b85d2"
//     "accept-language": "en-US,en;q=0.9"
//     "x-reddit-width": "411"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-session": "redacted"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "x-reddit-compression": "1"
//     "client-vendor-id": "fcf6b149-1386-458f-9379-04cd6e3d0460"
//     "accept-encoding": "gzip"
//     "content-length": "248"
//     "authorization": "Bearer redacted"
//     "x-reddit-device-id": "fcf6b149-1386-458f-9379-04cd6e3d0460"
// {"operationName":"GetAwardsForSubreddit","variables":{"subredditId":"t5_5s5qbl","thingId":"","includeSectionFields":true},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"80f71cc9610f3ff57310d00b45707789c8ddfd9dd86c4ea9f626b26fbf7b85d2"}}}
//
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "x-xss-protection": "1; mode=block"
//     "accept-ranges": "bytes"
//     "content-type": "application/json"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "x-ratelimit-reset": "205"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "date": "Wed, 29 Jan 2025 16:56:35 GMT"
//     "apollo-trace-id": "52a48aa85a4e23c36fd5b0f2fa744232"
//     "x-frame-options": "SAMEORIGIN"
//     "cache-control": "max-age=0, must-revalidate"
//     "x-content-type-options": "nosniff"
//     "server": "snooserv"
//     "content-length": "81"
//     "via": "1.1 varnish"
//     "x-reddit-session": "redacted"
//     "x-ratelimit-remaining": "1997.0"
//     "x-ratelimit-used": "3"
//     "vary": "origin"
//   body: Sized(81)
//  }
// {"data":{"subredditInfoById":{"__typename":"Subreddit","sortedUsableAwards":[]}}}


use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_awards_for_sub() -> HttpResponse {
    debug!("get_awards_for_sub");

    let rep = json! {{
        "data": {
            "subredditInfoById": {
                "__typename": "Subreddit",
                "sortedUsableAwards": []
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
