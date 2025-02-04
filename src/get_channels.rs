// {"operationName":"GetSubredditChannels","variables":{"subredditName":"AskReddit","includePostChannels":false},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"eaf6b70b60f085e941dc71f50286f309f97146032f1b170430c3d4d75369737a"}}}
// 
// {"data":{"subredditInfoByName":{"__typename":"Subreddit","channels":{"pageInfo":{"__typename":"PageInfo","hasNextPage":false,"endCursor":null},"edges":[]}}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_channels() -> HttpResponse {
    debug!("get_channels");

    let rep = json! {{
        "data": {
            "subredditInfoByName": {
                "__typename": "Subreddit",
                "channels": {
                    "pageInfo": {
                        "__typename": "PageInfo",
                        "hasNextPage": false,
                        "endCursor": null
                    },
                }
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
