// {"operationName":"AwardingTotalsForPost","variables":{"id":"t3_1id4wer"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"98eb6cacf8d2112b37d977da5079a9ba62ec86d549f7ad1444665c1019c55c0c"}}}
//
// {"data":{"postInfoById":{"awardings":[]}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_awarding_totals() -> HttpResponse {
    debug!("get_awarding_totals");

    let rep = json! {{
        "data": {
            "postInfoById": {
                "awardings": []
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
