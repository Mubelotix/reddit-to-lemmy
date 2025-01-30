// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-loid": "redacted"
//     "x-reddit-session": "redacted"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-compression": "1"
//     "x-apollo-operation-id": "eb48cdd031e6a6c3555ec3151aeb19f6b27cd24191ba2ebf673b06a8b789832b"
//     "content-type": "application/json; charset=utf-8"
//     "client-vendor-id": "2d2f7216-95fe-41c3-9cb9-8613a7ea4072"
//     "x-reddit-width": "411"
//     "content-length": "216"
//     "accept-language": "en-US,en;q=0.9"
//     "authorization": "Bearer redacted"
//     "x-apollo-operation-name": "UserProfile"
//     "x-reddit-device-id": "2d2f7216-95fe-41c3-9cb9-8613a7ea4072"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-qos": "down-rate-mbps=1.700"
//     "accept-encoding": "gzip"
//     "x-reddit-dpr": "2.625"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"

// {"operationName":"UserProfile","variables":{"name":"Mubelotix","includeTrophyCase":false},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"eb48cdd031e6a6c3555ec3151aeb19f6b27cd24191ba2ebf673b06a8b789832b"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "date": "Thu, 30 Jan 2025 00:00:03 GMT"
//     "x-reddit-session": "redacted"
//     "accept-ranges": "bytes"
//     "x-ratelimit-remaining": "1999.0"
//     "x-content-type-options": "nosniff"
//     "x-frame-options": "SAMEORIGIN"
//     "via": "1.1 varnish"
//     "vary": "origin"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "server": "snooserv"
//     "cache-control": "max-age=0, must-revalidate,max-age=0, must-revalidate"
//     "apollo-trace-id": "37247eedc7ae86ef200bf3b23a498dd4"
//     "x-ratelimit-reset": "597"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "x-xss-protection": "1; mode=block"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "content-type": "application/json"
//     "x-ratelimit-used": "1"
//     "content-length": "1885"
//   body: Sized(1885)
//  }
// {"data":{"redditorInfoByName":{"__typename":"Redditor","id":"t2_5xrogxaw","name":"Mubelotix","prefixedName":"u/Mubelotix","isFriend":false,"isEmployee":false,"isAcceptingChats":true,"isAcceptingFollowers":false,"isAcceptingPMs":true,"isVerified":true,"profile":{"createdAt":"2020-06-11T15:00:40.279000+0000","subscribersCount":0,"allowedPostTypes":[],"isUserBanned":false,"isContributor":false,"isDefaultIcon":false,"isDefaultBanner":true,"path":"/user/Mubelotix/","isNsfw":false,"title":"Mubelotix@jlai.lu","publicDescriptionText":"Cypherpunk","isSubscribed":false,"moderatorsInfo":{"edges":[{"node":{"id":"t2_5xrogxaw"}}]},"description":{"richtext":null},"socialLinks":[{"__typename":"SocialLink","id":"be84d6cd-c010-41bb-9775-ee1e6f036779","type":"CUSTOM","title":"Lemmy","handle":null,"outboundUrl":"https://jlai.lu/u/Mubelotix"}],"styles":{"icon":"https://styles.redditmedia.com/t5_2r8wgi/styles/profileIcon_snoo-nftv2_bmZ0X2VpcDE1NToxMzdfNDY2YTMzMDg4N2JkZjYyZDUzZjk2OGVhODI0NzkzMTUwZjA3NzYyZV82Njk_rare_4d6ae543-e1d8-4485-879d-fa6b1443b539-headshot.png?width=256\u0026height=256\u0026frame=1\u0026auto=webp\u0026crop=256:256,smart\u0026s=fb597a371a013483693ecd452d40e5eaeba4ca05","legacyPrimaryColor":null,"legacyIcon":{"url":"https://i.redd.it/snoovatar/avatars/nftv2_bmZ0X2VpcDE1NToxMzdfYzhkM2EzYTgzYmRlNWRhZDA2ZDQzNjY5NGUzZTIyYWMzZTY0ZDU3N18zOTA2MDEy_rare_fa763d6b-619c-4db4-87a3-9b47abd1eb53-headshot.png","dimensions":{"width":256,"height":256}},"profileBanner":null}},"profileExemptedExperiments":[],"karma":{"total":10223.0,"fromAwardsGiven":794.0,"fromAwardsReceived":308.0,"fromPosts":1146.0,"fromComments":7975.0},"snoovatarIcon":{"url":"https://i.redd.it/snoovatar/avatars/nftv2_bmZ0X2VpcDE1NToxMzdfYzhkM2EzYTgzYmRlNWRhZDA2ZDQzNjY5NGUzZTIyYWMzZTY0ZDU3N18zOTA2MDEy_rare_fa763d6b-619c-4db4-87a3-9b47abd1eb53.png"},"contributorPublicProfile":{"tier":"NON_CONTRIBUTOR"}}}}

use actix_web::{web::Json, HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::{person::GetPersonDetails, LemmyErrorType}, ClientOptions, LemmyClient, LemmyRequest};
use log::{debug, trace};
use serde::Deserialize;
use serde_json::json;
use crate::{get_jwt, GraphQlRequest, HackTraitCommunity, HackTraitPerson};
use GetProfileError::*;

#[derive(Debug)]
pub enum GetProfileError {
    Authentication,
    GetPerson(LemmyErrorType),
}

impl std::fmt::Display for GetProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            GetPerson(e) => write!(f, "GetPerson error: {e}"),
        }
    }
}

impl ResponseError for GetProfileError {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("GetProfileError: {self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProfileVariables {
    name: String,
    // include_trophy_case: bool,
}


pub async fn get_profile(request: HttpRequest, body: Json<GraphQlRequest<GetProfileVariables>>) -> Result<HttpResponse, GetProfileError> {
    debug!("get_profile: {:?}", body.variables);

    // Support distant-instance users
    
    let jwt = get_jwt(&request).ok_or(Authentication)?;

    let client = LemmyClient::new(ClientOptions {
        domain: String::from("jlai.lu"),
        secure: true
    });

    let details = client.get_person(LemmyRequest { body: GetPersonDetails {
        username: Some(body.variables.name.clone()),
        ..GetPersonDetails::default()
    }, jwt: Some(jwt.clone()) }).await.map_err(GetPerson)?;

    let mut social_links = Vec::new();
    if let Some(matrix) = &details.person_view.person.matrix_user_id {
        social_links.push(json! {{
            "__typename": "SocialLink",
            "id": "be84d6cd-c010-41bb-9775-ee1e6f036779",
            "type": "CUSTOM",
            "title": "Matrix",
            "handle": matrix,
            "outboundUrl": format!("https://matrix.to/#/{}", matrix)
        }});
    }

    let rep = json! {{
        "data": {
            "redditorInfoByName": {
                "__typename": "Redditor",
                "id": details.person_view.person.reddit_id(),
                "name": details.person_view.person.name,
                "prefixedName": details.person_view.person.prefixed_name(),
                "isFriend": false,
                "isEmployee": details.person_view.is_admin,
                "isAcceptingChats": true,
                "isAcceptingFollowers": false,
                "isAcceptingPMs": true,
                "isVerified": false,
                "profile": {
                    "createdAt": details.person_view.person.published,
                    "subscribersCount": 0,
                    "allowedPostTypes": [],
                    "isUserBanned": details.person_view.person.banned,
                    "isContributor": false,
                    "isDefaultIcon": details.person_view.person.avatar.is_none(),
                    "isDefaultBanner": details.person_view.person.banner.is_none(),
                    "path": details.person_view.person.path(),
                    "isNsfw": false,
                    "title": details.person_view.person.display_name.unwrap_or(details.person_view.person.name.clone()),
                    "publicDescriptionText": details.person_view.person.bio.unwrap_or_default(),
                    "isSubscribed": false,
                    "moderatorsInfo": {
                        "edges": details.moderates.iter().map(|view| json! {{
                            "node": {
                                "id": view.community.reddit_id()
                            }
                        }}).collect::<Vec<_>>() 
                    },
                    "description": {
                        "richtext": null
                    },
                    "socialLinks": social_links,
                    "styles": {
                        "icon": details.person_view.person.avatar,
                        "legacyPrimaryColor": null,
                        "legacyIcon": { "url": details.person_view.person.avatar, "dimensions": { "width": 256, "height": 256 } }, // FIXME: Dimensions
                        "profileBanner": { "url": details.person_view.person.banner, "dimensions": { "width": 256, "height": 256 } } // FIXME: Dimensions
                    },
                },
                "profileExemptedExperiments": [],
                "karma": {
                    "total": details.person_view.counts.post_score + details.person_view.counts.comment_score,
                    "fromAwardsGiven": 0,
                    "fromAwardsReceived": 0,
                    "fromPosts": details.person_view.counts.post_score,
                    "fromComments": details.person_view.counts.comment_score,
                },
                "snoovatarIcon": {
                    "url": details.person_view.person.avatar
                },
                "contributorPublicProfile": {
                    "tier": "NON_CONTRIBUTOR"
                }
            }
        }
    }};

    trace!("get_profile: {:?}", rep);
    Ok(HttpResponse::Ok().json(rep))
}

