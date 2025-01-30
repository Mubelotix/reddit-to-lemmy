// {"operationName":"GetCustomEmojisStatus","variables":{"subredditName":"oddlyspecific"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"cc00b8be74a4de8f52f85f998bc6095b84e7ec737efe586fd7bd13497c7fc08d"}}}
//
// {"data":{"subredditInfoByName":{"__typename":"Subreddit","customEmojisStatus":{"isEnabled":false}}}}


// {"operationName":"GetCustomEmojisStatus","variables":{"subredditName":"u_TeamAsana"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"cc00b8be74a4de8f52f85f998bc6095b84e7ec737efe586fd7bd13497c7fc08d"}}}
//
// {"data":{"subredditInfoByName":null}}

use actix_web::{web::Json, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use log::debug;

use crate::GraphQlRequest;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCustomEmojisVariables {
    subreddit_name: String
}

pub async fn get_custom_emojis(body: Json<GraphQlRequest<GetCustomEmojisVariables>>) -> HttpResponse {
    debug!("get_custom_emojis: {:?}", body.variables);

    let rep = match body.variables.subreddit_name.starts_with("u_") {
        true => json! {{
            "data": {
                "subredditInfoByName": null
            }
        }},
        false => json! {{
            "data": {
                "subredditInfoByName": {
                    "__typename": "Subreddit",
                    "customEmojisStatus": {
                        "isEnabled": false
                    }
                }
            }
        }}
    };

    HttpResponse::Ok().json(rep)
}
