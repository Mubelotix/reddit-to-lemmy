// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "x-reddit-compression": "1"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-reddit-width": "411"
//     "x-reddit-loid": "redacted"
//     "x-reddit-session": "redacted"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "content-length": "213"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-apollo-operation-id": "48cdb7ee9726cb111866c3c638d8ba619e3c4f8633b8e6803690a48576d3e40d"
//     "x-apollo-operation-name": "SearchChatMessageReactionIcons"
//     "accept-encoding": "gzip"
//     "content-type": "application/json; charset=utf-8"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "authorization": "Bearer redacted"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-dpr": "2.625"
//     "accept-language": "en-US,en;q=0.9"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"

// {"operationName":"SearchChatMessageReactionIcons","variables":{"query":"","first":100},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"48cdb7ee9726cb111866c3c638d8ba619e3c4f8633b8e6803690a48576d3e40d"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "server": "snooserv"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "content-length": "7424"
//     "via": "1.1 varnish"
//     "date": "Tue, 28 Jan 2025 17:18:34 GMT"
//     "vary": "origin"
//     "accept-ranges": "bytes"
//     "x-reddit-session": "redacted"
//     "x-ratelimit-remaining": "1985.0"
//     "x-ratelimit-used": "15"
//     "x-ratelimit-reset": "85"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//     "apollo-trace-id": "508a8baf18e3b8b27bced83e0c361dda"
//     "x-content-type-options": "nosniff"
//     "x-xss-protection": "1; mode=block"
//     "content-type": "application/json"
//     "x-frame-options": "SAMEORIGIN"
//   body: Sized(7424)
//  }
// {"data":{"searchChatMessageReactionIcons":{"edges":[{"node":{"url":"https://i.redd.it/8r21ukpfa7081.gif","key":"8r21ukpfa7081.gif","altText":"care","matrixUrl":"mxc://reddit.com/snoomoji_8r21ukpfa7081.gif"}},{"node":{"url":"https://i.redd.it/2o3aooqfa7081.gif","key":"2o3aooqfa7081.gif","altText":"wave","matrixUrl":"mxc://reddit.com/snoomoji_2o3aooqfa7081.gif"}},{"node":{"url":"https://i.redd.it/ytv3x0sfa7081.png","key":"ytv3x0sfa7081.png","altText":"doge","matrixUrl":"mxc://reddit.com/snoomoji_ytv3x0sfa7081.png"}},{"node":{"url":"https://i.redd.it/mag7v6tfa7081.gif","key":"mag7v6tfa7081.gif","altText":"cry","matrixUrl":"mxc://reddit.com/snoomoji_mag7v6tfa7081.gif"}},{"node":{"url":"https://i.redd.it/iuqmp7ufa7081.gif","key":"iuqmp7ufa7081.gif","altText":"disapproval","matrixUrl":"mxc://reddit.com/snoomoji_iuqmp7ufa7081.gif"}},{"node":{"url":"https://i.redd.it/zn7iubvfa7081.gif","key":"zn7iubvfa7081.gif","altText":"dizzy_face","matrixUrl":"mxc://reddit.com/snoomoji_zn7iubvfa7081.gif"}},{"node":{"url":"https://i.redd.it/qzl5vyxfa7081.gif","key":"qzl5vyxfa7081.gif","altText":"faceplam","matrixUrl":"mxc://reddit.com/snoomoji_qzl5vyxfa7081.gif"}},{"node":{"url":"https://i.redd.it/mi2jolzfa7081.gif","key":"mi2jolzfa7081.gif","altText":"feels_bad_man","matrixUrl":"mxc://reddit.com/snoomoji_mi2jolzfa7081.gif"}},{"node":{"url":"https://i.redd.it/k7ry7t1ga7081.gif","key":"k7ry7t1ga7081.gif","altText":"feels_good_man","matrixUrl":"mxc://reddit.com/snoomoji_k7ry7t1ga7081.gif"}},{"node":{"url":"https://i.redd.it/tspuf53ga7081.gif","key":"tspuf53ga7081.gif","altText":"flip_out","matrixUrl":"mxc://reddit.com/snoomoji_tspuf53ga7081.gif"}},{"node":{"url":"https://i.redd.it/evwks24ga7081.gif","key":"evwks24ga7081.gif","altText":"flushed","matrixUrl":"mxc://reddit.com/snoomoji_evwks24ga7081.gif"}},{"node":{"url":"https://i.redd.it/dfxygs4ga7081.gif","key":"dfxygs4ga7081.gif","altText":"give_upvote","matrixUrl":"mxc://reddit.com/snoomoji_dfxygs4ga7081.gif"}},{"node":{"url":"https://i.redd.it/ax7wu47ga7081.gif","key":"ax7wu47ga7081.gif","altText":"grimacing","matrixUrl":"mxc://reddit.com/snoomoji_ax7wu47ga7081.gif"}},{"node":{"url":"https://i.redd.it/uy83aa8ga7081.gif","key":"uy83aa8ga7081.gif","altText":"grin","matrixUrl":"mxc://reddit.com/snoomoji_uy83aa8ga7081.gif"}},{"node":{"url":"https://i.redd.it/t2r5xc9ga7081.gif","key":"t2r5xc9ga7081.gif","altText":"heart_eyes","matrixUrl":"mxc://reddit.com/snoomoji_t2r5xc9ga7081.gif"}},{"node":{"url":"https://i.redd.it/sjs1a2fyt7081.gif","key":"sjs1a2fyt7081.gif","altText":"heart_eyes_rainbow","matrixUrl":"mxc://reddit.com/snoomoji_sjs1a2fyt7081.gif"}},{"node":{"url":"https://i.redd.it/ksz4fmaga7081.gif","key":"ksz4fmaga7081.gif","altText":"hug","matrixUrl":"mxc://reddit.com/snoomoji_ksz4fmaga7081.gif"}},{"node":{"url":"https://i.redd.it/jvuspmbga7081.gif","key":"jvuspmbga7081.gif","altText":"joy","matrixUrl":"mxc://reddit.com/snoomoji_jvuspmbga7081.gif"}},{"node":{"url":"https://i.redd.it/mp9zclcga7081.gif","key":"mp9zclcga7081.gif","altText":"kissing_heart","matrixUrl":"mxc://reddit.com/snoomoji_mp9zclcga7081.gif"}},{"node":{"url":"https://i.redd.it/9ut7iedga7081.gif","key":"9ut7iedga7081.gif","altText":"laughing","matrixUrl":"mxc://reddit.com/snoomoji_9ut7iedga7081.gif"}},{"node":{"url":"https://i.redd.it/ul2w17ega7081.gif","key":"ul2w17ega7081.gif","altText":"money_face","matrixUrl":"mxc://reddit.com/snoomoji_ul2w17ega7081.gif"}},{"node":{"url":"https://i.redd.it/0ku6twega7081.gif","key":"0ku6twega7081.gif","altText":"neutral_face","matrixUrl":"mxc://reddit.com/snoomoji_0ku6twega7081.gif"}},{"node":{"url":"https://i.redd.it/cdnhvqfga7081.gif","key":"cdnhvqfga7081.gif","altText":"no_mouth","matrixUrl":"mxc://reddit.com/snoomoji_cdnhvqfga7081.gif"}},{"node":{"url":"https://i.redd.it/b5s6cohga7081.gif","key":"b5s6cohga7081.gif","altText":"put_back","matrixUrl":"mxc://reddit.com/snoomoji_b5s6cohga7081.gif"}},{"node":{"url":"https://i.redd.it/av9z8iiga7081.gif","key":"av9z8iiga7081.gif","altText":"rage","matrixUrl":"mxc://reddit.com/snoomoji_av9z8iiga7081.gif"}},{"node":{"url":"https://i.redd.it/00brcfjga7081.gif","key":"00brcfjga7081.gif","altText":"scream","matrixUrl":"mxc://reddit.com/snoomoji_00brcfjga7081.gif"}},{"node":{"url":"https://i.redd.it/pleyoikga7081.gif","key":"pleyoikga7081.gif","altText":"shrug","matrixUrl":"mxc://reddit.com/snoomoji_pleyoikga7081.gif"}},{"node":{"url":"https://i.redd.it/m7uy86lga7081.gif","key":"m7uy86lga7081.gif","altText":"sleep","matrixUrl":"mxc://reddit.com/snoomoji_m7uy86lga7081.gif"}},{"node":{"url":"https://i.redd.it/8kw138jyt7081.gif","key":"8kw138jyt7081.gif","altText":"slightly_smiling","matrixUrl":"mxc://reddit.com/snoomoji_8kw138jyt7081.gif"}},{"node":{"url":"https://i.redd.it/wbrgz1nga7081.gif","key":"wbrgz1nga7081.gif","altText":"smile","matrixUrl":"mxc://reddit.com/snoomoji_wbrgz1nga7081.gif"}},{"node":{"url":"https://i.redd.it/fsg2a1oga7081.gif","key":"fsg2a1oga7081.gif","altText":"snoo","matrixUrl":"mxc://reddit.com/snoomoji_fsg2a1oga7081.gif"}},{"node":{"url":"https://i.redd.it/8zx5ixoga7081.gif","key":"8zx5ixoga7081.gif","altText":"sob","matrixUrl":"mxc://reddit.com/snoomoji_8zx5ixoga7081.gif"}},{"node":{"url":"https://i.redd.it/t5bqwxdyt7081.gif","key":"t5bqwxdyt7081.gif","altText":"stuck_out_tongue","matrixUrl":"mxc://reddit.com/snoomoji_t5bqwxdyt7081.gif"}},{"node":{"url":"https://i.redd.it/xjls1pqga7081.gif","key":"xjls1pqga7081.gif","altText":"sunglasses","matrixUrl":"mxc://reddit.com/snoomoji_xjls1pqga7081.gif"}},{"node":{"url":"https://i.redd.it/7rc03trga7081.gif","key":"7rc03trga7081.gif","altText":"surprise","matrixUrl":"mxc://reddit.com/snoomoji_7rc03trga7081.gif"}},{"node":{"url":"https://i.redd.it/d45pfmsga7081.gif","key":"d45pfmsga7081.gif","altText":"sweat","matrixUrl":"mxc://reddit.com/snoomoji_d45pfmsga7081.gif"}},{"node":{"url":"https://i.redd.it/g6akcitga7081.gif","key":"g6akcitga7081.gif","altText":"sweat_smile","matrixUrl":"mxc://reddit.com/snoomoji_g6akcitga7081.gif"}},{"node":{"url":"https://i.redd.it/t1djdguga7081.gif","key":"t1djdguga7081.gif","altText":"table","matrixUrl":"mxc://reddit.com/snoomoji_t1djdguga7081.gif"}},{"node":{"url":"https://i.redd.it/19b5q4vga7081.gif","key":"19b5q4vga7081.gif","altText":"table_flip","matrixUrl":"mxc://reddit.com/snoomoji_19b5q4vga7081.gif"}},{"node":{"url":"https://i.redd.it/4zhlw4iyt7081.gif","key":"4zhlw4iyt7081.gif","altText":"thinking_face_hmm","matrixUrl":"mxc://reddit.com/snoomoji_4zhlw4iyt7081.gif"}},{"node":{"url":"https://i.redd.it/3s0glewga7081.gif","key":"3s0glewga7081.gif","altText":"thumbs_down","matrixUrl":"mxc://reddit.com/snoomoji_3s0glewga7081.gif"}},{"node":{"url":"https://i.redd.it/7url39xga7081.gif","key":"7url39xga7081.gif","altText":"thumbs_up","matrixUrl":"mxc://reddit.com/snoomoji_7url39xga7081.gif"}},{"node":{"url":"https://i.redd.it/d2kn6yxga7081.gif","key":"d2kn6yxga7081.gif","altText":"trollface","matrixUrl":"mxc://reddit.com/snoomoji_d2kn6yxga7081.gif"}},{"node":{"url":"https://i.redd.it/foyijyyga7081.gif","key":"foyijyyga7081.gif","altText":"upvote","matrixUrl":"mxc://reddit.com/snoomoji_foyijyyga7081.gif"}},{"node":{"url":"https://i.redd.it/3i2arwzga7081.gif","key":"3i2arwzga7081.gif","altText":"wink","matrixUrl":"mxc://reddit.com/snoomoji_3i2arwzga7081.gif"}},{"node":{"url":"https://i.redd.it/79opsq0ha7081.gif","key":"79opsq0ha7081.gif","altText":"yummy","matrixUrl":"mxc://reddit.com/snoomoji_79opsq0ha7081.gif"}}]}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn search_message_reactions() -> HttpResponse {
    debug!("search_message_reactions");

    let rep = json! {{
        "data": {
            "searchChatMessageReactionIcons": {
                "edges": []
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
