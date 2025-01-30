// {"operationName":"MarketingNudges","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"38856a88e2407db78f8a217698fbefec8cb4cdc26b62770eb8f2e04d986f2d6e"}}}
// 
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
