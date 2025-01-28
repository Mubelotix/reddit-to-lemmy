// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-apollo-operation-name": "GetAllVaults"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "accept-language": "en-US,en;q=0.9"
//     "x-apollo-operation-id": "2ab376466d1a57a79f0ffb2b6cbdd0ead3c3b26cbdcf352c1360c1df60ed12cb"
//     "x-reddit-width": "411"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "authorization": "Bearer redacted"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "accept-encoding": "gzip"
//     "x-reddit-loid": "redacted"
//     "x-reddit-compression": "1"
//     "x-reddit-session": "redacted"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "content-type": "application/json; charset=utf-8"
//     "content-length": "217"
//     "x-reddit-dpr": "2.625"

// {"operationName":"GetAllVaults","variables":{"provider":"ethereum","includeInactive":true},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"2ab376466d1a57a79f0ffb2b6cbdd0ead3c3b26cbdcf352c1360c1df60ed12cb"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "x-ratelimit-reset": "83"
//     "date": "Tue, 28 Jan 2025 17:18:36 GMT"
//     "apollo-trace-id": "99c35a7ccc47f118fc25515a69b269e0"
//     "vary": "origin"
//     "x-content-type-options": "nosniff"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "x-xss-protection": "1; mode=block"
//     "server": "snooserv"
//     "accept-ranges": "bytes"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "content-length": "175"
//     "x-frame-options": "SAMEORIGIN"
//     "content-type": "application/json"
//     "x-reddit-session": "redacted"
//     "x-ratelimit-remaining": "1980.0"
//     "x-ratelimit-used": "20"
//     "via": "1.1 varnish"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//   body: Sized(175)
//  }
// {"data":{"vault":{"addresses":[{"provider":"ethereum","address":"0x17463b2D658C7019b46d0c21AaB05d44E7313770","createdAt":"2021-07-10T07:17:22.000000+0000","isActive":true}]}}}

use actix_web::HttpResponse;
use serde_json::json;

pub async fn get_vaults() -> HttpResponse {
    HttpResponse::Ok()
        .json(json! {{
            "data": {
                "vault": {
                    "addresses": []
                }
            }
        }})
}
