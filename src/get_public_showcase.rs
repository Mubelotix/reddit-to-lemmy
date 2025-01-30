// {"operationName":"GetPublicShowcaseOfCurrentUser","variables":{"count":10},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"57aa1ad3034ed38d2c453cd36a9c9371d6eb506f22049aedb9e70f8389be43a4"}}}
// 
// {"data":{"identity":{"displayedCollectibleItemsState":"ENABLED","redditor":{"displayedCollectibleItems":{"__typename":"DisplayedCollectibleItemsConnection","edges":[{"node":{"__typename":"DisplayedCollectibleItem","isVisible":true,"item":{"id":"nft_eip155:137_466a330887bdf62d53f968ea824793150f07762e_669","name":"The Singularity #670","drop":{"size":null},"images":[{"__typename":"MediaSource","url":"https://i.redd.it/snoovatar/avatars/basic/d3b58fe3-f35c-4da0-879b-0e85d68f1fd6.png","dimensions":{"width":552,"height":736}}]}}},{"node":{"__typename":"DisplayedCollectibleItem","isVisible":true,"item":{"id":"nft_eip155:137_3b477a6b1be236628b08839e1e8cf8ba8d93589a_5729","name":"Eagles #5730","drop":{"size":2000000},"images":[{"__typename":"MediaSource","url":"https://i.redd.it/snoovatar/avatars/basic/320c1246-ef3e-43a0-b08d-74e7c47cd1a9.png","dimensions":{"width":552,"height":736}}]}}},{"node":{"__typename":"DisplayedCollectibleItem","isVisible":true,"item":{"id":"nft_eip155:137_c8d3a3a83bde5dad06d436694e3e22ac3e64d577_3906012","name":"Pixel Placers #53787","drop":{"size":null},"images":[{"__typename":"MediaSource","url":"https://i.redd.it/snoovatar/avatars/basic/9d0d13cc-e7aa-4678-8459-e2b073efe731.png","dimensions":{"width":552,"height":736}}]}}}]}}}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_public_showcase() -> HttpResponse {
    debug!("get_public_showcase");

    let rep = json! {{
        "data": {
            "identity": {
                "displayedCollectibleItemsState": "DISABLED",
                "redditor": {
                    "displayedCollectibleItems": {
                        "edges": []
                    }
                }
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}

