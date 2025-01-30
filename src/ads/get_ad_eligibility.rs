// {"operationName":"AdEligibilityForUser","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"39a5a68f81a20f9f7dafb72f017022d0ced17493d564e2fa8ff1286774eab6e6"}}}
// 
// {"data":{"adEligibility":{"userAdEligibility":"ELIGIBLE"}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_ad_eligibility() -> HttpResponse {
    debug!("get_ad_eligibility");

    let rep = json! {{
        "data": {
            "adEligibility": {
                "userAdEligibility": "NOT_ELIGIBLE_CONTEXT" // Value randomly found in the source code. Not sure what this means nor what it does.
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
