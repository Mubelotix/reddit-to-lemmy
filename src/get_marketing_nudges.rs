// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-loid": "redacted"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-apollo-operation-id": "38856a88e2407db78f8a217698fbefec8cb4cdc26b62770eb8f2e04d986f2d6e"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-apollo-operation-name": "MarketingNudges"
//     "content-type": "application/json; charset=utf-8"
//     "authorization": "Bearer redacted"
//     "x-reddit-width": "411"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "accept-language": "en-US,en;q=0.9"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "accept-encoding": "gzip"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "x-reddit-compression": "1"
//     "content-length": "176"
//     "x-reddit-dpr": "2.625"
//     "x-reddit-session": "redacted"
// 
// {"operationName":"MarketingNudges","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"38856a88e2407db78f8a217698fbefec8cb4cdc26b62770eb8f2e04d986f2d6e"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "content-length": "40"
//     "x-ratelimit-used": "12"
//     "apollo-trace-id": "476c1a57e1934e353c51ef5a4095abd6"
//     "date": "Tue, 28 Jan 2025 17:18:34 GMT"
//     "via": "1.1 varnish"
//     "x-ratelimit-remaining": "1988.0"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "server": "snooserv"
//     "x-xss-protection": "1; mode=block"
//     "x-frame-options": "SAMEORIGIN"
//     "vary": "origin"
//     "content-type": "application/json"
//     "accept-ranges": "bytes"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "x-content-type-options": "nosniff"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "x-ratelimit-reset": "85"
//   body: Sized(40)
//  }
// {"data":{"econMarketing":{"nudges":[]}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_marketing_nudges() -> HttpResponse {
    debug!("get_marketing_nudges");

    let rep = json! {{
        "data": {
            "econMarketing": {
                "nudges": []
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
