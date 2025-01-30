// {"operationName":"RegisterMobilePushToken","variables":{"authTokens":["invalid-token","redacted"],"pushToken":"redacted","deviceId":"6918b40d-1a50-4478-8b05-ad96845c1ae3","timezoneName":"Europe/Paris","timestamp":"2025-01-28T17:18:29.615","language":"en-US"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"35313b81bfbb621c426b2f2ec276db3db476a5d8362a1ddbeec982a01dc74977"}}}
// 
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
