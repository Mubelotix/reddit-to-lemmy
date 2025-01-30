// {"operationName":"GetEarnedGoldBalance","variables":{"environment":"PRODUCTION"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"ff32d82d205c0ddf80c6369656c435ae84c45d6120388feebbc474036ed25a82"}}}
//
// {"data":{"identity":{"goldBalances":{"earned":{"available":0,"total":0}}}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_earned_gold_balance() -> HttpResponse {
    debug!("get_earned_gold_balance");

    let rep = json! {{
        "data": {
            "identity": {
                "goldBalances": {
                    "earned": {
                        "available": 0,
                        "total": 0
                    }
                }
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
