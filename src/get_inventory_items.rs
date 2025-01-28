// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-apollo-operation-name": "GetInventoryItemsByIds"
//     "content-type": "application/json; charset=utf-8"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-dpr": "2.625"
//     "x-apollo-operation-id": "b8d35af133fcfe112baec9c898333f7cf217ede2a467ab60467a6f839c572263"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "authorization": "Bearer redacted"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-width": "411"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-loid": "redacted"
//     "x-reddit-compression": "1"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "content-length": "256"
//     "accept-language": "en-US,en;q=0.9"
//     "accept-encoding": "gzip"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "x-reddit-session": "redacted"
// 
// {"operationName":"GetInventoryItemsByIds","variables":{"ids":["nft_eip155:137_c8d3a3a83bde5dad06d436694e3e22ac3e64d577_3906012"]},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"b8d35af133fcfe112baec9c898333f7cf217ede2a467ab60467a6f839c572263"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "x-xss-protection": "1; mode=block"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "x-frame-options": "SAMEORIGIN"
//     "server": "snooserv"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//     "x-content-type-options": "nosniff"
//     "x-ratelimit-reset": "85"
//     "via": "1.1 varnish"
//     "content-type": "application/json"
//     "date": "Tue, 28 Jan 2025 17:18:34 GMT"
//     "vary": "origin"
//     "x-ratelimit-used": "11"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "content-length": "1222"
//     "apollo-trace-id": "0e82baf3c94e352baed527b6f7deca22"
//     "x-ratelimit-remaining": "1989.0"
//     "x-reddit-session": "redacted"
//     "accept-ranges": "bytes"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//   body: Sized(1222)
//  }
// {"data":{"inventoryItems":{"edges":[{"node":{"__typename":"InventoryItem","id":"nft_eip155:137_c8d3a3a83bde5dad06d436694e3e22ac3e64d577_3906012","name":"Pixel Placers #53787","tags":["MINTED"],"serialNumber":"53787","owner":{"id":"t2_5xrogxaw","displayName":"Mubelotix"},"artist":null,"benefits":{"avatarOutfit":{"id":"01GK5KTRN8RFZT3W675AJFNWN2","preRenderImage":{"url":"https://i.redd.it/snoovatar/avatars/basic/9d0d13cc-e7aa-4678-8459-e2b073efe731.png"},"backgroundImage":{"url":"https://i.redd.it/snoovatar/submission_snoo_assets/_Ly7aSs63Pk_.png"}},"avatarUtilities":[]},"drop":{"size":null,"rarity":null},"nft":{"contractAddress":"c8d3a3a83bde5dad06d436694e3e22ac3e64d577","title":"Pixel Placers #53787","description":"Redditors love a blank canvas.","externalUrls":["https://polygonscan.com/token/0xc8d3a3a83bde5dad06d436694e3e22ac3e64d577?a=3906012"],"series":"Pixel Placers","mintedAt":"2022-12-17T15:31:24.000000+0000","tokenUrl":"https://reddit.infura-ipfs.io/ipfs/QmPWUhx4rH2gZvkwuqTTNt1H11hE2ySC1yJ2Gv5QvDkXqn/3906012.json","tokenId":"3906012","imageUrl":"https://reddit.infura-ipfs.io/ipfs/QmPgxQF7fYaHPoxd5Njz57qgGwnwJVNo4Vc4cvQp63yX31","wallet":{"address":"17463b2d658c7019b46d0c21aab05d44e7313770"}}}}]}}}

use actix_web::{HttpResponse, HttpResponseBuilder};
use awc::http::StatusCode;
use serde_json::json;

pub async fn get_inventory_items() -> HttpResponse {
    HttpResponseBuilder::new(StatusCode::OK)
        .json(json! {{
            "data": {
                "inventoryItems": {
                    "edges": []
                }
            }
        }})
}
