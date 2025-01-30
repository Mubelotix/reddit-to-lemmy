// {"operationName":"GetAccountPreferences","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"af026643cf85baffdb73ff191578778f42eb90998c8f49ffa6701bd67a6b0fa1"}}}
// 
// {"data":{"identity":{"preferences":{"isAdPersonalizationAllowed":false,"isClickTrackingEnabled":false,"defaultCommentSort":"CONFIDENCE","geopopular":"","isProfileHiddenFromRobots":false,"isSuggestedSortIgnored":false,"mediaThumbnailVisibility":"SUBREDDIT","isNsfwMediaBlocked":true,"isNsfwContentShown":true,"isNsfwSearchEnabled":true,"isLocationBasedRecommendationEnabled":false,"surveyLastSeenAt":null,"isThirdPartyAdPersonalizationAllowed":false,"isThirdPartySiteAdPersonalizationAllowed":false,"isThirdPartyInfoAdPersonalizationAllowed":false,"isThirdPartySiteDataPersonalizedContentAllowed":false,"isTopKarmaSubredditsShown":false,"acceptPrivateMessagesFrom":"EVERYONE","isEmailOptedOut":false,"isOnlinePresenceShown":false,"isFeedRecommendationsEnabled":true,"countryCode":"XZ","isFollowersEnabled":false,"isEmailDigestEnabled":false,"isShowFollowersCountEnabled":false,"isSmsNotificationsEnabled":false,"minCommentScore":-4,"isMachineTranslationImmersive":"UNSET","hiddenSubredditIds":[],"isHideAllContribution":false,"isHideProfileNsfw":false}}}}

use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, ClientOptions, LemmyClient, LemmyRequest};
use serde_json::json;
use crate::get_jwt;
use log::{debug, trace};
use GetPreferencesError::*;

#[derive(Debug)]
pub enum GetPreferencesError {
    Authentication,
    GetSite(LemmyErrorType),
    MissingUser,
}

impl std::fmt::Display for GetPreferencesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            GetSite(e) => write!(f, "GetSite error: {e}"),
            MissingUser => write!(f, "Missing user error"),
        }
    }
}

impl ResponseError for GetPreferencesError {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("GetAccountError: {self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_preferences(request: HttpRequest) -> Result<HttpResponse, GetPreferencesError> {
    debug!("get_preferences");

    let jwt = get_jwt(&request).ok_or(Authentication)?;

    let client = LemmyClient::new(ClientOptions {
        domain: String::from("jlai.lu"),
        secure: true
    });

    let site = client.get_site(LemmyRequest { body: (), jwt: Some(jwt.clone()) }).await.map_err(GetSite)?;
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
                "preferences": {
                    "isAdPersonalizationAllowed": false,
                    "isClickTrackingEnabled": false,
                    "defaultCommentSort": "CONFIDENCE",
                    "geopopular": "",
                    "isProfileHiddenFromRobots": false,
                    "isSuggestedSortIgnored": false,
                    "mediaThumbnailVisibility": "SUBREDDIT",
                    "isNsfwMediaBlocked": !my_user.local_user_view.local_user.show_nsfw,
                    "isNsfwContentShown": my_user.local_user_view.local_user.show_nsfw,
                    "isNsfwSearchEnabled": my_user.local_user_view.local_user.show_nsfw,
                    "isLocationBasedRecommendationEnabled": false,
                    "surveyLastSeenAt": null,
                    "isThirdPartyAdPersonalizationAllowed":	false,
                    "isThirdPartySiteAdPersonalizationAllowed":	false,
                    "isThirdPartyInfoAdPersonalizationAllowed":	false,
                    "isThirdPartySiteDataPersonalizedContentAllowed": false,
                    "isTopKarmaSubredditsShown": false,
                    "acceptPrivateMessagesFrom": "EVERYONE",
                    "isEmailOptedOut": false,
                    "isOnlinePresenceShown": false,
                    "isFeedRecommendationsEnabled": true,
                    "countryCode": "US",
                    "isFollowersEnabled": false,
                    "isEmailDigestEnabled": false,
                    "isShowFollowersCountEnabled": false,
                    "isSmsNotificationsEnabled": false,
                    "minCommentScore": -4,
                    "isMachineTranslationImmersive": "UNSET",
                    "hiddenSubredditIds": [], // TODO
                    "isHideAllContribution": false,
                    "isHideProfileNsfw": !my_user.local_user_view.local_user.show_nsfw,
                }
            }
        }
    }};
    
    trace!("get_preferences response: {}", serde_json::to_string(&rep).unwrap_or_default());
    Ok(HttpResponse::Ok().json(rep))
}
