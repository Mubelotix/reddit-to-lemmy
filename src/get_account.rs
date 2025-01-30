
// {"operationName":"GetAccount","variables":{"subscriptionType":"Premium","isPremiumApiMigrationEnabled":false,"includeTrophyCase":true},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"8f381d92420ae0d8fa23ca310865d4fc60d122e8265dd60db86dd0c39a96c5a5"}}}
//
// {"data":{"identity":{"id":"t2_5xrogxaw","createdAt":"2020-06-11T15:00:40.279000+0000","email":"mubelotix@gmail.com","isEmailPermissionRequired":true,"isSuspended":false,"isModerator":false,"suspensionExpiresAt":null,"isEmailVerified":true,"isPasswordSet":true,"isForcePasswordReset":false,"coins":0,"isNameEditable":false,"isSubredditCreationAllowed":true,"preferences":{"isTopKarmaSubredditsShown":false},"econSubscriptions":[],"linkedIdentities":[{"issuer":"GOOGLE"}],"phoneNumber":{"code":null,"number":null},"inbox":{"unreadCount":183},"modMail":{"isUnread":false},"redditor":{"id":"t2_5xrogxaw","name":"Mubelotix","prefixedName":"u/Mubelotix","isEmployee":false,"isFriend":false,"isPremiumMember":false,"isProfileHiddenFromSearchEngines":false,"isAcceptingChats":true,"isAcceptingFollowers":false,"cakeDayOn":"2020-06-11","snoovatarIcon":{"url":"https://i.redd.it/snoovatar/avatars/nftv2_bmZ0X2VpcDE1NToxMzdfYzhkM2EzYTgzYmRlNWRhZDA2ZDQzNjY5NGUzZTIyYWMzZTY0ZDU3N18zOTA2MDEy_rare_fa763d6b-619c-4db4-87a3-9b47abd1eb53.png"},"profile":{"id":"t5_2r8wgi","createdAt":"2020-06-11T15:00:40.279000+0000","isUserBanned":false,"isDefaultBanner":true,"path":"/user/Mubelotix/","socialLinks":[{"__typename":"SocialLink","id":"be84d6cd-c010-41bb-9775-ee1e6f036779","type":"CUSTOM","title":"Lemmy","handle":null,"outboundUrl":"https://jlai.lu/u/Mubelotix"}],"isSubscribed":false,"isTopListingAllowed":true,"allowedPostTypes":["LINK","IMAGE","VIDEO","TEXT","SPOILER","POLL","GALLERY"],"description":{"richtext":null},"isNsfw":false,"title":"Mubelotix@jlai.lu","subscribersCount":0,"isDefaultIcon":false,"isContributor":false,"publicDescriptionText":"Cypherpunk","moderatorsInfo":{"edges":[{"node":{"id":"t2_5xrogxaw"}}]},"styles":{"icon":"https://styles.redditmedia.com/t5_2r8wgi/styles/profileIcon_snoo-nftv2_bmZ0X2VpcDE1NToxMzdfNDY2YTMzMDg4N2JkZjYyZDUzZjk2OGVhODI0NzkzMTUwZjA3NzYyZV82Njk_rare_4d6ae543-e1d8-4485-879d-fa6b1443b539-headshot.png?width=256\u0026height=256\u0026frame=1\u0026auto=webp\u0026crop=256:256,smart\u0026s=fb597a371a013483693ecd452d40e5eaeba4ca05","legacyPrimaryColor":null,"legacyIcon":{"url":"https://i.redd.it/snoovatar/avatars/nftv2_bmZ0X2VpcDE1NToxMzdfYzhkM2EzYTgzYmRlNWRhZDA2ZDQzNjY5NGUzZTIyYWMzZTY0ZDU3N18zOTA2MDEy_rare_fa763d6b-619c-4db4-87a3-9b47abd1eb53-headshot.png","dimensions":{"width":256,"height":256}},"profileBanner":null}},"karma":{"total":10223.0,"fromAwardsGiven":794.0,"fromAwardsReceived":308.0,"fromPosts":1146.0,"fromComments":7975.0},"trophyCase":{"name":"Trophies","totalUnlocked":10}}}}}

use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, ClientOptions, LemmyClient, LemmyRequest};
use log::{debug, trace};
use serde_json::json;
use crate::{get_jwt, HackTraitPerson};
use GetAccountError::*;

#[derive(Debug)]
pub enum GetAccountError {
    Authentication,
    GetSite(LemmyErrorType),
    UnreadCount(LemmyErrorType),
    MissingUser,
}

impl std::fmt::Display for GetAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            GetSite(e) => write!(f, "GetSite error: {e}"),
            UnreadCount(e) => write!(f, "UnreadCount error: {e}"),
            MissingUser => write!(f, "Missing user error"),
        }
    }
}

impl ResponseError for GetAccountError {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("GetAccountError: {self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_account(request: HttpRequest) -> Result<HttpResponse, GetAccountError> {
    debug!("get_account");
    
    let jwt = get_jwt(&request).ok_or(Authentication)?;

    let client = LemmyClient::new(ClientOptions {
        domain: String::from("jlai.lu"),
        secure: true
    });

    let site = client.get_site(LemmyRequest { body: (), jwt: Some(jwt.clone()) }).await.map_err(GetSite)?;
    let unread_count = client.unread_count(LemmyRequest { body: (), jwt: Some(jwt) }).await.map_err(UnreadCount)?;
    let my_user = site.my_user.ok_or(MissingUser)?;

    let mut social_links = Vec::new();
    if let Some(matrix) = &my_user.local_user_view.person.matrix_user_id {
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
            "identity": {
                "id": my_user.local_user_view.person.reddit_id(),
                "createdAt": my_user.local_user_view.person.published,
                "email": my_user.local_user_view.local_user.email,
                "isEmailPermissionRequired": false,
                "isSuspended": my_user.local_user_view.person.banned,
                "isModerator": !my_user.moderates.is_empty(),
                "suspensionExpiresAt": my_user.local_user_view.person.ban_expires,
                "isEmailVerified": my_user.local_user_view.local_user.email_verified,
                "isPasswordSet": true,
                "isForcePasswordReset": false,
                "coins": 0,
                "isNameEditable": true,
                "isSubredditCreationAllowed": true,
                "preferences": {
                    "isTopKarmaSubredditsShown": false
                },
                "econSubscriptions": [],
                "linkedIdentities": [],
                "phoneNumber": { "code": null, "number": null },
                "inbox": { "unreadCount": unread_count.replies + unread_count.mentions + unread_count.private_messages },
                "modMail": { "isUnread": false }, // TODO
                "redditor": {
                    "id": my_user.local_user_view.person.reddit_id(),
                    "name": my_user.local_user_view.person.name,
                    "prefixedName": my_user.local_user_view.person.prefixed_name(),
                    "isEmployee": my_user.local_user_view.local_user.admin,
                    "isFriend": false,
                    "isPremiumMember": true,
                    "isProfileHiddenFromSearchEngines": false,
                    "isAcceptingChats": true,
                    "isAcceptingFollowers": false,
                    "cakeDayOn": my_user.local_user_view.person.published.format("%Y-%m-%d").to_string(),
                    "snoovatarIcon": { "url": my_user.local_user_view.person.avatar },
                    "profile": {
                        "id": my_user.local_user_view.person.reddit_id(),
                        "createdAt": my_user.local_user_view.person.published,
                        "isUserBanned": my_user.local_user_view.person.banned,
                        "isDefaultBanner": my_user.local_user_view.person.banner.is_none(),
                        "path": my_user.local_user_view.person.path(),
                        "socialLinks": social_links,
                        "isSubscribed": false,
                        "isTopListingAllowed": true,
                        "allowedPostTypes": ["LINK", "IMAGE", "VIDEO", "TEXT", "SPOILER", "POLL", "GALLERY"],
                        "description": { "richtext": null },
                        "isNsfw": false,
                        "title": my_user.local_user_view.person.display_name,
                        "subscribersCount": 0,
                        "isDefaultIcon": my_user.local_user_view.person.avatar.is_none(),
                        "isContributor": false,
                        "publicDescriptionText": my_user.local_user_view.person.bio,
                        "moderatorsInfo": { "edges": [
                            { "node": { "id": my_user.local_user_view.person.reddit_id() } }
                        ] },
                        "styles": {
                            "icon": my_user.local_user_view.person.avatar,
                            "legacyPrimaryColor": null,
                            "legacyIcon": { "url": my_user.local_user_view.person.avatar, "dimensions": { "width": 256, "height": 256 } }, // FIXME: Dimensions
                            "profileBanner": { "url": my_user.local_user_view.person.banner, "dimensions": { "width": 256, "height": 256 } } // FIXME: Dimensions
                        },
                        "karma": {
                            "total": my_user.local_user_view.counts.post_score + my_user.local_user_view.counts.comment_score,
                            "fromAwardsGiven": 0,
                            "fromAwardsReceived": 0,
                            "fromPosts": my_user.local_user_view.counts.post_score,
                            "fromComments": my_user.local_user_view.counts.comment_score
                        },
                        "trophyCase": {
                            "name": "Trophies",
                            "totalUnlocked": 0
                        }
                    }
                }
            }
        }
    }};

    trace!("get_account: {:?}", rep);
    Ok(HttpResponse::Ok().json(rep))
}
