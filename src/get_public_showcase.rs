// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-apollo-operation-name": "GetPublicShowcaseOfCurrentUser"
//     "content-length": "201"
//     "accept-language": "en-US,en;q=0.9"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-apollo-operation-id": "57aa1ad3034ed38d2c453cd36a9c9371d6eb506f22049aedb9e70f8389be43a4"
//     "authorization": "Bearer redacted"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-compression": "1"
//     "x-reddit-loid": "redacted"
//     "x-reddit-dpr": "2.625"
//     "x-reddit-session": "redacted"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-width": "411"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "content-type": "application/json; charset=utf-8"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "accept-encoding": "gzip"

// {"operationName":"GetPublicShowcaseOfCurrentUser","variables":{"count":10},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"57aa1ad3034ed38d2c453cd36a9c9371d6eb506f22049aedb9e70f8389be43a4"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "content-length": "1268"
//     "x-content-type-options": "nosniff"
//     "content-type": "application/json"
//     "x-ratelimit-reset": "85"
//     "x-ratelimit-remaining": "1984.0"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate,private, s-maxage=0, max-age=0, must-revalidate"
//     "accept-ranges": "bytes"
//     "server": "snooserv"
//     "vary": "origin"
//     "x-frame-options": "SAMEORIGIN"
//     "x-ratelimit-used": "16"
//     "x-xss-protection": "1; mode=block"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "x-reddit-session": "redacted"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "via": "1.1 varnish"
//     "apollo-trace-id": "c629bcdb42aa72a50883cc290e5e4be6"
//     "date": "Tue, 28 Jan 2025 17:18:34 GMT"
//   body: Sized(1268)
//  }
// {"data":{"identity":{"displayedCollectibleItemsState":"ENABLED","redditor":{"displayedCollectibleItems":{"__typename":"DisplayedCollectibleItemsConnection","edges":[{"node":{"__typename":"DisplayedCollectibleItem","isVisible":true,"item":{"id":"nft_eip155:137_466a330887bdf62d53f968ea824793150f07762e_669","name":"The Singularity #670","drop":{"size":null},"images":[{"__typename":"MediaSource","url":"https://i.redd.it/snoovatar/avatars/basic/d3b58fe3-f35c-4da0-879b-0e85d68f1fd6.png","dimensions":{"width":552,"height":736}}]}}},{"node":{"__typename":"DisplayedCollectibleItem","isVisible":true,"item":{"id":"nft_eip155:137_3b477a6b1be236628b08839e1e8cf8ba8d93589a_5729","name":"Eagles #5730","drop":{"size":2000000},"images":[{"__typename":"MediaSource","url":"https://i.redd.it/snoovatar/avatars/basic/320c1246-ef3e-43a0-b08d-74e7c47cd1a9.png","dimensions":{"width":552,"height":736}}]}}},{"node":{"__typename":"DisplayedCollectibleItem","isVisible":true,"item":{"id":"nft_eip155:137_c8d3a3a83bde5dad06d436694e3e22ac3e64d577_3906012","name":"Pixel Placers #53787","drop":{"size":null},"images":[{"__typename":"MediaSource","url":"https://i.redd.it/snoovatar/avatars/basic/9d0d13cc-e7aa-4678-8459-e2b073efe731.png","dimensions":{"width":552,"height":736}}]}}}]}}}}}

use actix_web::{HttpResponse, HttpResponseBuilder};
use awc::http::StatusCode;
use serde_json::json;

pub async fn get_public_showcase() -> HttpResponse {
    HttpResponseBuilder::new(StatusCode::OK)
        .json(json! {{
            "data": {
                "identity": {
                    "displayedCollectibleItemsState": "DISABLED",
                    "redditor": {
                        "displayedCollectibleItems": {
                            "edges": []
                        }
                    }
                }
            }
        }})
}

