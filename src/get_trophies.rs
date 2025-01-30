// {"operationName":"ProfileTrophies","variables":{"profileName":"Different-You7646"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"afd499fd984d22dba654280c5a59467a2288a37c6e650648e4cb35fba88960ba"}}}
//
// {"data":{"redditorInfoByName":{"__typename":"Redditor","trophies":[{"description":null,"icon70Url":"https://www.redditstatic.com/awards2/3_year_club-70.png","grantedAt":"2024-10-03T19:02:50.508000+0000","name":"Three-Year Club","trophyId":null,"awardId":null,"url":null}]}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_trophies() -> HttpResponse {
    debug!("get_trophies");

    let rep = json! {{
        "data": {
            "redditorInfoByName": {
                "__typename": "Redditor",
                "trophies": []
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
