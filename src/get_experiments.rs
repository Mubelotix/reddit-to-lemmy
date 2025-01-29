// ientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-dpr": "2.625"
//     "x-reddit-compression": "1"
//     "authorization": "Bearer redacted"
//     "x-reddit-qos": "down-rate-mbps=1.700"
//     "client-vendor-id": "fcf6b149-1386-458f-9379-04cd6e3d0460"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "accept-language": "en-US,en;q=0.9"
//     "x-apollo-operation-name": "UsernameAndExperiments"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "content-type": "application/json; charset=utf-8"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "accept-encoding": "gzip"
//     "x-reddit-device-id": "fcf6b149-1386-458f-9379-04cd6e3d0460"
//     "x-apollo-operation-id": "9bd622448d88f4232245ffdf82943a8ad22727e8d93a7666ac907cac077fd88a"
//     "x-reddit-width": "411"
//     "content-length": "357"
// 
// {"operationName":"UsernameAndExperiments","variables":{"inputs":[{"field":"device_id","value":"fcf6b149-1386-458f-9379-04cd6e3d0460"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}]},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"9bd622448d88f4232245ffdf82943a8ad22727e8d93a7666ac907cac077fd88a"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "accept-ranges": "bytes"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "x-ratelimit-remaining": "499.0"
//     "x-frame-options": "SAMEORIGIN"
//     "content-type": "application/json"
//     "x-reddit-session": "redacted"
//     "via": "1.1 varnish"
//     "x-reddit-loid": "redacted"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "vary": "Accept-Encoding"
//     "server": "snooserv"
//     "x-ratelimit-reset": "289"
//     "content-length": "167981"
//     "cache-control": "max-age=0, must-revalidate"
//     "x-content-type-options": "nosniff"
//     "x-xss-protection": "1; mode=block"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "x-ratelimit-used": "1"
//     "date": "Wed, 29 Jan 2025 16:55:10 GMT"
//   body: Sized(167981)
//  }
// 

use actix_web::HttpResponse;
use log::debug;

// Note: The endpoint should return the username but let's try to just return the experiments for now

pub async fn get_experiments() -> HttpResponse {
    debug!("get_experiments");

    let rep = include_bytes!("config/experiments.json");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(rep.to_vec())
}
