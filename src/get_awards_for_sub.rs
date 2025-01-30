// {"operationName":"GetAwardsForSubreddit","variables":{"subredditId":"t5_5s5qbl","thingId":"","includeSectionFields":true},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"80f71cc9610f3ff57310d00b45707789c8ddfd9dd86c4ea9f626b26fbf7b85d2"}}}
//
// {"data":{"subredditInfoById":{"__typename":"Subreddit","sortedUsableAwards":[]}}}


use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_awards_for_sub() -> HttpResponse {
    debug!("get_awards_for_sub");

    let rep = json! {{
        "data": {
            "subredditInfoById": {
                "__typename": "Subreddit",
                "sortedUsableAwards": []
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
