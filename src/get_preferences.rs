// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "content-length": "182"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-compression": "1"
//     "x-apollo-operation-name": "GetAccountPreferences"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-dpr": "2.625"
//     "content-type": "application/json; charset=utf-8"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "x-apollo-operation-id": "af026643cf85baffdb73ff191578778f42eb90998c8f49ffa6701bd67a6b0fa1"
//     "accept-encoding": "gzip"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-reddit-session": "redacted"
//     "accept-language": "en-US,en;q=0.9"
//     "authorization": "Bearer redacted"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-width": "411"
//     "x-reddit-loid": "redacted"
// 
// {"operationName":"GetAccountPreferences","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"af026643cf85baffdb73ff191578778f42eb90998c8f49ffa6701bd67a6b0fa1"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "x-xss-protection": "1; mode=block"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//     "content-type": "application/json"
//     "x-reddit-session": "redacted"
//     "date": "Tue, 28 Jan 2025 17:18:34 GMT"
//     "accept-ranges": "bytes"
//     "x-ratelimit-remaining": "1992.0"
//     "x-ratelimit-used": "8"
//     "via": "1.1 varnish"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "content-length": "1054"
//     "x-content-type-options": "nosniff"
//     "x-ratelimit-reset": "85"
//     "apollo-trace-id": "e32c14179757d4ccf5fb0ce352a0e5d0"
//     "vary": "origin"
//     "server": "snooserv"
//     "x-frame-options": "SAMEORIGIN"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//   body: Sized(1054)
//  }
// {"data":{"identity":{"preferences":{"isAdPersonalizationAllowed":false,"isClickTrackingEnabled":false,"defaultCommentSort":"CONFIDENCE","geopopular":"","isProfileHiddenFromRobots":false,"isSuggestedSortIgnored":false,"mediaThumbnailVisibility":"SUBREDDIT","isNsfwMediaBlocked":true,"isNsfwContentShown":true,"isNsfwSearchEnabled":true,"isLocationBasedRecommendationEnabled":false,"surveyLastSeenAt":null,"isThirdPartyAdPersonalizationAllowed":false,"isThirdPartySiteAdPersonalizationAllowed":false,"isThirdPartyInfoAdPersonalizationAllowed":false,"isThirdPartySiteDataPersonalizedContentAllowed":false,"isTopKarmaSubredditsShown":false,"acceptPrivateMessagesFrom":"EVERYONE","isEmailOptedOut":false,"isOnlinePresenceShown":false,"isFeedRecommendationsEnabled":true,"countryCode":"XZ","isFollowersEnabled":false,"isEmailDigestEnabled":false,"isShowFollowersCountEnabled":false,"isSmsNotificationsEnabled":false,"minCommentScore":-4,"isMachineTranslationImmersive":"UNSET","hiddenSubredditIds":[],"isHideAllContribution":false,"isHideProfileNsfw":false}}}}

use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, ClientOptions, LemmyClient, LemmyRequest};
use serde_json::json;
use crate::get_jwt;
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
    
    Ok(HttpResponse::Ok().json(rep))
}
