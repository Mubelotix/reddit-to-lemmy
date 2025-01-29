// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-apollo-operation-id": "f07de258c54537e24d7856080f662c1b1268210251e5789c8c08f20d76cc8ab2"
//     "x-reddit-session": "redacted"
//     "x-reddit-width": "411"
//     "x-reddit-device-id": "fcf6b149-1386-458f-9379-04cd6e3d0460"
//     "x-reddit-loid": "redacted"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "x-reddit-compression": "1"
//     "content-length": "173"
//     "client-vendor-id": "fcf6b149-1386-458f-9379-04cd6e3d0460"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "accept-encoding": "gzip"
//     "accept-language": "en-US,en;q=0.9"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "content-type": "application/json; charset=utf-8"
//     "x-reddit-dpr": "2.625"
//     "x-apollo-operation-name": "UserLocation"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "authorization": "Bearer redacted"
//     "x-reddit-qos": "down-rate-mbps=1.700"
// {"operationName":"UserLocation","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"f07de258c54537e24d7856080f662c1b1268210251e5789c8c08f20d76cc8ab2"}}}
// 
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "accept-ranges": "bytes"
//     "server": "snooserv"
//     "date": "Wed, 29 Jan 2025 16:56:34 GMT"
//     "x-ratelimit-reset": "205"
//     "x-content-type-options": "nosniff"
//     "apollo-trace-id": "3551ee78d8a51dadb710cf715bc3502f"
//     "via": "1.1 varnish"
//     "x-xss-protection": "1; mode=block"
//     "content-type": "application/json"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "vary": "origin"
//     "x-ratelimit-used": "6"
//     "x-ratelimit-remaining": "1994.0"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "content-length": "141"
//     "x-frame-options": "SAMEORIGIN"
//   body: Sized(141)
//  }
// {"data":{"userLocation":{"countryCode":"FR","regionCode":"NOR","cityCode":"NOTRE-DAME-DE-BONDEVILLE","cityUtf8":"NOTRE-DAME-DE-BONDEVILLE"}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_location() -> HttpResponse {
    debug!("get_location");

    let rep = json! {{
        "data": {
            "userLocation": {
                "countryCode": "US",
                "regionCode": "CA",
                "cityCode": "LOS-ANGELES",
                "cityUtf8": "LOS-ANGELES"
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
