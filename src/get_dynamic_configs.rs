// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-session": "redacted"
//     "content-type": "application/json; charset=utf-8"
//     "x-apollo-operation-name": "AllDynamicConfigs"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-apollo-operation-id": "a4056084f78a61e765d0c5b654a8ccbff4923732fc69c9ee907a57ef4c1bb2ce"
//     "x-reddit-qos": "down-rate-mbps=1.700"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-device-id": "fcf6b149-1386-458f-9379-04cd6e3d0460"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "content-length": "178"
//     "x-reddit-compression": "1"
//     "authorization": "Bearer redacted"
//     "x-reddit-dpr": "2.625"
//     "accept-encoding": "gzip"
//     "x-reddit-loid": "redacted"
//     "x-reddit-width": "411"
//     "accept-language": "en-US,en;q=0.9"
//     "client-vendor-id": "fcf6b149-1386-458f-9379-04cd6e3d0460"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
// {"operationName":"AllDynamicConfigs","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"a4056084f78a61e765d0c5b654a8ccbff4923732fc69c9ee907a57ef4c1bb2ce"}}}
//
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "date": "Wed, 29 Jan 2025 16:56:35 GMT"
//     "x-frame-options": "SAMEORIGIN"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "cache-control": "max-age=0, must-revalidate"
//     "x-ratelimit-reset": "204"
//     "vary": "origin"
//     "apollo-trace-id": "0254486bb9f5cc60a79f5d7f1e84b4bb"
//     "x-content-type-options": "nosniff"
//     "x-xss-protection": "1; mode=block"
//     "via": "1.1 varnish"
//     "x-ratelimit-remaining": "1991.0"
//     "server": "snooserv"
//     "content-length": "157342"
//     "content-type": "application/json"
//     "accept-ranges": "bytes"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "x-ratelimit-used": "9"
//     "x-reddit-session": "redacted"
//   body: Sized(157342)
//  }

use actix_web::HttpResponse;
use log::debug;

pub async fn get_dynamic_configs() -> HttpResponse {
    debug!("get_dynamic_configs");

    let rep = include_bytes!("config/dynamic_configs.json");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(rep.to_vec())
}
