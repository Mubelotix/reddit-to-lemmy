// {"operationName":"CommentTreeAds","variables":{"postId":"t3_1idlf9a","sortType":"CONFIDENCE","adContext":{"distance":null,"layout":"CARD","deviceAdId":"871986d3-16ac-4ea8-b0be-569cebf340b2","clientSignalSessionData":{"adsSeenCount":5,"totalPostsSeenCount":47,"sessionStartTime":"2025-01-30T15:31:54.079"},"feedSlotIndexData":{},"forceAds":{}},"includeAwards":false,"includeEconPromos":false,"includeCurrentUserAwards":false,"includeStillMediaAltText":false,"includeMediaAuth":false,"includePostStats":false,"includeSubredditInPosts":true,"includeNewPixelTrackingFields":true,"includeExcludedExperimentsField":true,"includeExtendedVideoAsset":false},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"3e567d7edcb749b4ef29ce9377bd02f5dd3ab535ef1b01819d6dfae15a2b51f8"}}}
//
// {"data":{"postInfoById":{"__typename":"SubredditPost","commentTreeAds":[]}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_comment_tree_ads() -> HttpResponse {
    debug!("get_comment_tree_ads");

    let rep = json! {{
        "data": {
            "postInfoById": {
                "__typename": "SubredditPost",
                "commentTreeAds": []
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
