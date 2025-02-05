// {"operationName":"SearchTypeaheadByType","variables":{"query":"ask","filters":[{"key":"nsfw","value":"1"}],"productSurface":"android","searchInput":{"queryId":"ace1b860-61c0-4b5f-be87-b59c37a5e223","correlationId":"ffef2df6-95a6-4eab-87d0-fb7195707c36","originPageType":"home","structureType":"search"},"limit":5,"includeUsers":true,"includeEligibleMoment":false},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"fe372c97004a67146b1af1310e411e5573252159b35515db24528920b5c9a5bf"}}}
//
// {"data":{"search":{"typeaheadByType":{"feedMetadata":{"appliedFilters":{"edges":[{"node":{"key":"nsfw","value":"1"}}]},"queryTags":[]},"subreddits":[{"__typename":"Subreddit","id":"t5_2s30g","name":"AskMen","type":"PUBLIC","prefixedName":"r/AskMen","publicDescriptionText":"We don't read the rules, but we'll post anyways.","isQuarantined":false,"subscribersCount":6536360.0,"isNsfw":false,"isSubscribed":true,"styles":{"icon":"https://styles.redditmedia.com/t5_2s30g/styles/communityIcon_wpxjh8fuvcw51.png?width=256\u0026height=256\u0026frame=1\u0026auto=webp\u0026crop=256:256,smart\u0026s=f36f87bbf07eede24fbc19ad5000a00222343115","legacyIcon":{"url":"https://a.thumbs.redditmedia.com/LH9Y1HS41ygKhs8OoNOXyoS3ovy7x4LBkxci05XMAZ0.png"},"primaryColor":"#C0C0C0","legacyPrimaryColor":"#0079D3"}},{"__typename":"Subreddit","id":"t5_2zkfk","name":"AskFrance","type":"PUBLIC","prefixedName":"r/AskFrance","publicDescriptionText":"Demandez ce que vous voulez à la communauté française de Reddit","isQuarantined":false,"subscribersCount":330355,"isNsfw":false,"isSubscribed":true,"styles":{"icon":"https://styles.redditmedia.com/t5_2zkfk/styles/communityIcon_3s5fyq6t9bn81.jpg?width=256\u0026height=256\u0026frame=1\u0026auto=webp\u0026crop=256:256,smart\u0026s=af0dccf484caa16575f3501f1181d590365d032e","legacyIcon":null,"primaryColor":"#014980","legacyPrimaryColor":"#24A0ED"}},{"__typename":"Subreddit","id":"t5_2qhlj","name":"ask","type":"PUBLIC","prefixedName":"r/ask","publicDescriptionText":"This is a place to ask specific, closed-ended, non divisive questions.","isQuarantined":false,"subscribersCount":1155484.0,"isNsfw":false,"isSubscribed":false,"styles":{"icon":"https://styles.redditmedia.com/t5_2qhlj/styles/communityIcon_keez0blzwen31.jpg?width=256\u0026height=256\u0026frame=1\u0026auto=webp\u0026crop=256:256,smart\u0026s=0861ec4c374cd15820bfd86a92c952e8a56aace8","legacyIcon":{"url":"https://b.thumbs.redditmedia.com/ynpviSWdi_rPZS1R2sUib6EgUuNm1o6RHLVoTjuxp1M.png"},"primaryColor":"#D40000","legacyPrimaryColor":"#545452"}},{"__typename":"Subreddit","id":"t5_2qh1i","name":"AskReddit","type":"PUBLIC","prefixedName":"r/AskReddit","publicDescriptionText":"r/AskReddit is the place to ask and answer thought-provoking questions.","isQuarantined":false,"subscribersCount":51243009.0,"isNsfw":false,"isSubscribed":false,"styles":{"icon":"https://styles.redditmedia.com/t5_2qh1i/styles/communityIcon_p6kb2m6b185b1.png?width=256\u0026height=256\u0026frame=1\u0026auto=webp\u0026crop=256:256,smart\u0026s=abc8188748dbdea7183a757c54491b38a6e578d0","legacyIcon":{"url":"https://b.thumbs.redditmedia.com/LSHrisQApf1H5F8nWShTx3_KjTOMc3R_ss3kx3XAyXQ.png"},"primaryColor":"#646D73","legacyPrimaryColor":"#222222"}},{"__typename":"Subreddit","id":"t5_2rxrw","name":"AskWomen","type":"PUBLIC","prefixedName":"r/AskWomen","publicDescriptionText":"AskWomen: A subreddit dedicated to asking women questions about their thoughts, lives, and experiences; providing a place where all women can comfortably and candidly share their responses in a non-judgmental space. As part of our commitment to that mission, the AskWomen subreddit is curated to promote respectful and on-topic discussions, and not serve as a debate subreddit.","isQuarantined":false,"subscribersCount":5547962.0,"isNsfw":true,"isSubscribed":false,"styles":{"icon":"https://styles.redditmedia.com/t5_2rxrw/styles/communityIcon_yhvvesso2f7b1.jpg?width=256\u0026height=256\u0026frame=1\u0026auto=webp\u0026crop=256:256,smart\u0026s=fcb768e049b6c2eaeccad49db2c1def2e592cb64","legacyIcon":{"url":"https://b.thumbs.redditmedia.com/T3VnE1TL0SupQ7jrCWLXQhYi3Cc-OtNSp98i2ilLSTc.png"},"primaryColor":"#FAD5E6","legacyPrimaryColor":"#FF66AC"}}],"profiles":[]}}}}


use actix_web::{web::Json, HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::{lemmy_db_schema::{ListingType, SearchType, SubscribedType}, site::Search, LemmyErrorType}, LemmyRequest};
use log::{debug, trace};
use serde::Deserialize;
use serde_json::json;
use crate::{get_lemmy_client, markdown_to_text, GraphQl, HackTraitCommunity};
use SearchError::*;

#[derive(Debug)]
pub enum SearchError {
    Authentication,
    Search(LemmyErrorType),
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            Search(e) => write!(f, "Search error: {e}"),
        }
    }
}

impl ResponseError for SearchError {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("SearchCommunitiesError: {self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchVariables {
    query: String,
    limit: i64,
    // TODO: include_users: bool, 
}

pub async fn search(request: HttpRequest, body: Json<GraphQl<SearchVariables>>) -> Result<HttpResponse, SearchError> {
    debug!("search: {:?}", body.variables);
    
    let (jwt, client) = get_lemmy_client(&request).ok_or(Authentication)?;

    let results = client.search(LemmyRequest { body: Search {
        q: body.variables.query.clone(),
        type_: Some(SearchType::Communities),
        listing_type: Some(ListingType::All),
        limit: Some(body.variables.limit),
        ..Default::default()
    }, jwt: Some(jwt.clone()) }).await.map_err(Search)?;

    let rep = json! {{
        "data": {
            "search": {
                "typeaheadByType": {
                    "feedMetadata": {
                        "appliedFilters": {
                            "edges": []
                        },
                        "queryTags": []
                    },
                    "subreddits": results.communities.into_iter().map(|view| json! {{
                        "__typename": "Subreddit",
                        "id": view.community.reddit_id(),
                        "name": view.community.name,
                        "type": view.community.reddit_type(),
                        "prefixedName": view.community.prefixed_name(),
                        "publicDescriptionText": view.community.description.as_deref().map(markdown_to_text).unwrap_or_default(),
                        "isQuarantined": false,
                        "subscribersCount": view.counts.subscribers,
                        "isNsfw": false,
                        "isSubscribed": view.subscribed != SubscribedType::NotSubscribed,
                        "styles": {
                            "icon": view.community.icon.as_ref().map(|i| i.to_string()).unwrap_or_default(),
                            "legacyIcon": {
                                "url": view.community.icon.as_ref().map(|i| i.to_string()).unwrap_or_default()
                            },
                            "primaryColor": null,
                            "legacyPrimaryColor": null
                        }
                    }}).collect::<Vec<_>>(),
                }
            }
        }
    }};

    trace!("search: {}", serde_json::to_string(&rep).unwrap_or_default());
    Ok(HttpResponse::Ok().json(rep))
}
