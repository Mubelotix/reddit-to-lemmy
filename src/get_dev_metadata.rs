// {"operationName":"GetDevPlatformMetadata","variables":{"subredditId":"t5_2wjlc","mimetype":"application/protobuf"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"78e9fe51edcd5a013f7f2f5af95ddec5af332dc38305de78cd9979c8e01a63e9"}}}
// 
// {"data":{"subredditInfoById":{"__typename":"Subreddit","devPlatformMetadata":""}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_dev_metadata() -> HttpResponse {
    debug!("get_dev_metadata");

    let rep = json! {{
        "data": {
            "subredditInfoById": {
                "__typename": "Subreddit",
                "devPlatformMetadata": ""
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
