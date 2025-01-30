// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-compression": "1"
//     "x-reddit-qos": "down-rate-mbps=1.700"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "x-apollo-operation-name": "ExposeExperiments"
//     "content-length": "4734"
//     "x-reddit-width": "411"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "authorization": "Bearer redacted"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "x-reddit-session": "redacted"
//     "client-vendor-id": "2d2f7216-95fe-41c3-9cb9-8613a7ea4072"
//     "x-reddit-device-id": "2d2f7216-95fe-41c3-9cb9-8613a7ea4072"
//     "accept-encoding": "gzip"
//     "content-type": "application/json; charset=utf-8"
//     "x-reddit-dpr": "2.625"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-apollo-operation-id": "0c708ab4affad577faaa59a47c0f107da3b61215915fc86bfecfe61c36cc4da8"
//     "accept-language": "en-US,en;q=0.9"

// {"operationName":"ExposeExperiments","variables":{"inputs":[{"experimentName":"android_improve_composer_prompt","experimentVersion":"4","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"join_the_conversation"},{"experimentName":"android_ads_ad_caching_fix_v2","experimentVersion":"4","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"caching_pdp_fix"},{"experimentName":"android_post_stats_revamp","experimentVersion":"6","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_x_mr_awarded_comments_treatment","experimentVersion":"11","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_ads_in_comments","experimentVersion":"48","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_pdp_post_unit_navbar","experimentVersion":"7","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_sct_upgrade_m1","experimentVersion":"9","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_ads_in_comments_dedupe","experimentVersion":"4","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"bizx_android_show_follower_count_settings","experimentVersion":"9","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_chat_create_chat_fab","experimentVersion":"5","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_cdd_improving_chat_empty_state","experimentVersion":"14","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_chat_threads_view","experimentVersion":"11","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_community_chat_live_bar_discovery","experimentVersion":"26","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_chat_discover_chats_in_the_list","experimentVersion":"4","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_chat_collapse_offensive_messages","experimentVersion":"6","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_pn_inbox_tab_view_event_bug_fix","experimentVersion":"14","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"}]},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"0c708ab4affad577faaa59a47c0f107da3b61215915fc86bfecfe61c36cc4da8"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "x-ratelimit-reset": "377"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "content-length": "241"
//     "x-xss-protection": "1; mode=block"
//     "date": "Thu, 30 Jan 2025 00:33:42 GMT"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//     "content-type": "application/json"
//     "x-ratelimit-remaining": "1999.0"
//     "server": "snooserv"
//     "via": "1.1 varnish"
//     "x-frame-options": "SAMEORIGIN"
//     "x-reddit-session": "redacted"
//     "accept-ranges": "bytes"
//     "x-content-type-options": "nosniff"
//     "x-ratelimit-used": "1"
//     "x-reddit-loid": "redacted"
//     "vary": "Accept-Encoding"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//   body: Sized(241)
//  }
// {"data":{"exposeExperimentBatch":{"payloads":[{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true},{"ok":true}]}}}

use actix_web::{web::Json, HttpResponse};
use serde::Deserialize;
use serde_json::{json, Value};
use log::debug;

use crate::GraphQlRequest;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExposeExperimentsVariables {
    inputs: Vec<Value>,
}

pub async fn expose_experiments(body: Json<GraphQlRequest<ExposeExperimentsVariables>>) -> HttpResponse {
    debug!("expose_experiments");

    let rep = json! {{
        "data": {
            "exposeExperimentBatch": {
                "payloads": (0..body.variables.inputs.len()).map(|_| json!({ "ok": true })).collect::<Vec<_>>(),
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
