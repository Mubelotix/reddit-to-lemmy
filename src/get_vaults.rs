// {"operationName":"GetAllVaults","variables":{"provider":"ethereum","includeInactive":true},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"2ab376466d1a57a79f0ffb2b6cbdd0ead3c3b26cbdcf352c1360c1df60ed12cb"}}}
// 
// {"data":{"vault":{"addresses":[{"provider":"ethereum","address":"0x17463b2D658C7019b46d0c21AaB05d44E7313770","createdAt":"2021-07-10T07:17:22.000000+0000","isActive":true}]}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_vaults() -> HttpResponse {
    debug!("get_vaults");

    let rep = json! {{
        "data": {
            "vault": {
                "addresses": []
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
