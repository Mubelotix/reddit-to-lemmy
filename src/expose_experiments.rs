// {"operationName":"ExposeExperiments","variables":{"inputs":[{"experimentName":"android_improve_composer_prompt","experimentVersion":"4","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"join_the_conversation"},{"experimentName":"android_ads_ad_caching_fix_v2","experimentVersion":"4","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"caching_pdp_fix"},{"experimentName":"android_post_stats_revamp","experimentVersion":"6","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_x_mr_awarded_comments_treatment","experimentVersion":"11","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_ads_in_comments","experimentVersion":"48","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_pdp_post_unit_navbar","experimentVersion":"7","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_sct_upgrade_m1","experimentVersion":"9","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_ads_in_comments_dedupe","experimentVersion":"4","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"bizx_android_show_follower_count_settings","experimentVersion":"9","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_chat_create_chat_fab","experimentVersion":"5","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_cdd_improving_chat_empty_state","experimentVersion":"14","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_chat_threads_view","experimentVersion":"11","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_community_chat_live_bar_discovery","experimentVersion":"26","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_chat_discover_chats_in_the_list","experimentVersion":"4","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_chat_collapse_offensive_messages","experimentVersion":"6","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"},{"experimentName":"android_pn_inbox_tab_view_event_bug_fix","experimentVersion":"14","targetingInputs":[{"field":"device_id","value":"2d2f7216-95fe-41c3-9cb9-8613a7ea4072"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}],"variant":"enabled"}]},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"0c708ab4affad577faaa59a47c0f107da3b61215915fc86bfecfe61c36cc4da8"}}}
// 
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
