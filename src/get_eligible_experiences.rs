// {"operationName":"GetEligibleUxExperiences","variables":{"experienceInputs":["COMMUNITY_ONBOARDING","AWARDS_PROMO"],"advancedConfiguration":{"eligibleExperienceOverrides":[]},"clientContext":{"pageType":"COMMUNITY","subredditId":"t5_2s30g"},"includeSavedProperties":false},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"02d965634c34062edd230b6a4edab85bfe48a1777e5fb5d0c94ef1f8f4d54f9d"}}}
// 
// {"data":{"eligibleUxExperiences":[]}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_eligible_experiences() -> HttpResponse {
    debug!("get_eligible_experiences");

    let rep = json! {{
        "data": {
            "eligibleUxExperiences": []
        }
    }};

    HttpResponse::Ok().json(rep)
}

