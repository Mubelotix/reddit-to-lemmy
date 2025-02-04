// {"operationName":"GetChatChannelsRecommendations","variables":{"postId":"t3_1ihliem"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"141b4caeb026b547165325ac3b389190c45423c9c76bcf122ea8a79bb689db9d"}}}
// 
// {"data":{"postInfoById":{"__typename":"SubredditPost","recommendedChatChannels":{"analyticsInfo":{"recommendationAlgorithm":null},"recommendedChannels":[]}}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_channel_recommendations() -> HttpResponse {
    debug!("get_channel_recommendations");

    let rep = json! {{
        "data": {
            "postInfoById": {
                "__typename": "SubredditPost",
                "recommendedChatChannels": {
                    "analyticsInfo": {
                        "recommendationAlgorithm": null
                    },
                    "recommendedChannels": []
                }
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
