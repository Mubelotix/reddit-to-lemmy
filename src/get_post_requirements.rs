// {"operationName":"GetPostRequirements","variables":{"subredditId":"t5_2s30g"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"0465f6238026c63c1688696b323fb987de6372459b8f873bf36fd6a8a4e38421"}}}
//
// {"data":{"subredditInfoById":{"__typename":"Subreddit","postRequirements":{"__typename":"PostRequirements","bodyBlacklistedStrings":[],"bodyRegexes":[],"bodyRequiredStrings":[],"bodyRestrictionPolicy":"NONE","domainBlacklist":[],"domainWhitelist":[],"galleryCaptionsRequirement":"NONE","galleryMaxItems":null,"galleryMinItems":null,"galleryUrlsRequirement":"NONE","guidelinesText":"Read the Rules!\n\nCommon rules users break when submitting a post:\n\n1. Title must be a clear, concise question.\n\n2. Cannot be FAQ. Search with keywords for any recent posts before making a new one.\n\n3. Do not post seeking advice for personal issues or situations.\n\nAgain, these are just some common rule breaks. Read through all the rules before posting!","isFlairRequired":false,"linkRepostAge":null,"linkRestrictionPolicy":"NONE","titleBlacklistedStrings":[],"titleRegexes":[],"titleRequiredStrings":[],"titleTextMaxLength":null,"titleTextMinLength":null}}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_post_requirements() -> HttpResponse {
    debug!("get_post_requirements");

    let rep = json! {{
        "data": {
            "subredditInfoById": {
                "__typename": "Subreddit",
                "postRequirements": {
                    "__typename": "PostRequirements",
                    "bodyBlacklistedStrings":[],
                    "bodyRegexes":[],
                    "bodyRequiredStrings":[],
                    "bodyRestrictionPolicy":"NONE",
                    "domainBlacklist":[],
                    "domainWhitelist":[],
                    "galleryCaptionsRequirement":"NONE",
                    "galleryMaxItems":null,
                    "galleryMinItems":null,
                    "galleryUrlsRequirement":"NONE",
                    "guidelinesText":"",
                    "isFlairRequired":false,
                    "linkRepostAge":null,
                    "linkRestrictionPolicy":"NONE",
                    "titleBlacklistedStrings":[],
                    "titleRegexes":[],
                    "titleRequiredStrings":[],
                    "titleTextMaxLength":null,
                    "titleTextMinLength":null
                }
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
