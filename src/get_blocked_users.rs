// ClientRequest HTTP/1.1 POST https://gql-fed.reddit.com/
//   headers:
//     "accept-language": "en-US,en;q=0.9"
//     "accept": "multipart/mixed;deferSpec=20220824, application/json"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-qos": "down-rate-mbps=77.994"
//     "content-type": "application/json; charset=utf-8"
//     "x-apollo-operation-id": "64203eb8f6dc3576b80332b2542e6cf87bfe6900174c1098865e693f57737544"
//     "content-length": "192"
//     "x-reddit-session": "redacted"
//     "authorization": "Bearer redacted"
//     "x-reddit-loid": "redacted"
//     "x-apollo-operation-name": "BlockedRedditors"
//     "x-reddit-compression": "1"
//     "x-dev-ad-id": "871986d3-16ac-4ea8-b0be-569cebf340b2"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "accept-encoding": "gzip"
//     "x-reddit-device-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "device-name": "Google;sdk_gphone64_x86_64"
//     "x-reddit-dpr": "2.625"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-reddit-width": "411"
// 
// {"operationName":"BlockedRedditors","variables":{"pageSize":1000},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"64203eb8f6dc3576b80332b2542e6cf87bfe6900174c1098865e693f57737544"}}}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "x-xss-protection": "1; mode=block"
//     "x-ratelimit-reset": "85"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 0.1, \"failure_fraction\": 0.1}"
//     "cache-control": "private, s-maxage=0, max-age=0, must-revalidate"
//     "date": "Tue, 28 Jan 2025 17:18:34 GMT"
//     "x-frame-options": "SAMEORIGIN"
//     "content-type": "application/json"
//     "x-ratelimit-remaining": "1991.0"
//     "x-content-type-options": "nosniff"
//     "server": "snooserv"
//     "via": "1.1 varnish"
//     "vary": "origin"
//     "accept-ranges": "bytes"
//     "x-ratelimit-used": "9"
//     "x-reddit-session": "redacted"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "content-length": "4342"
//     "apollo-trace-id": "acd80e237de0857bdc1411e5dcc8a014"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//   body: Sized(4342)
//  }
// {"data":{"identity":{"blockedRedditorsInfo":{"pageInfo":{"__typename":"PageInfo","hasNextPage":false,"endCursor":"dDJfZmVtNzFicg=="},"edges":[{"node":{"id":"t2_4vgf5774"}},{"node":{"id":"t2_ffh8y3pa"}},{"node":{"id":"t2_evxodoq"}},{"node":{"id":"t2_fsanfy5c"}},{"node":{"id":"t2_10ow0k"}},{"node":{"id":"t2_4437ngsf"}},{"node":{"id":"t2_hmljqp3k"}},{"node":{"id":"t2_i0124ces"}},{"node":{"id":"t2_5ya359tg"}},{"node":{"id":"t2_5k19w"}},{"node":{"id":"t2_9bltfbg"}},{"node":{"id":"t2_dylmairi"}},{"node":{"id":"t2_21olm20"}},{"node":{"id":"t2_eqwo7"}},{"node":{"id":"t2_ldz2vwiy"}},{"node":{"id":"t2_3hicrmk2"}},{"node":{"id":"t2_lvsgv6ev"}},{"node":{"id":"t2_7e6e6ycz"}},{"node":{"id":"t2_1h2d1n"}},{"node":{"id":"t2_gi993kqp"}},{"node":{"id":"t2_rq49o"}},{"node":{"id":"t2_4c2jd"}},{"node":{"id":"t2_9fr8tdyp"}},{"node":{"id":"t2_37f11mf"}},{"node":{"id":"t2_16wya3"}},{"node":{"id":"t2_8pnv472h"}},{"node":{"id":"t2_16hyoe"}},{"node":{"id":"t2_bsoifpv8"}},{"node":{"id":"t2_a67uzs5u"}},{"node":{"id":"t2_ol2q3jfd"}},{"node":{"id":"t2_8xcw9sqe"}},{"node":{"id":"t2_efeh7"}},{"node":{"id":"t2_mr9v1j3r"}},{"node":{"id":"t2_7zcpw"}},{"node":{"id":"t2_h72r3n73"}},{"node":{"id":"t2_btmv6m3x"}},{"node":{"id":"t2_izu2l"}},{"node":{"id":"t2_lg17vvre"}},{"node":{"id":"t2_3f6q43xz"}},{"node":{"id":"t2_1l4rlbz"}},{"node":{"id":"t2_dpob1pwc"}},{"node":{"id":"t2_4f8xdvec"}},{"node":{"id":"t2_1veuqn1i"}},{"node":{"id":"t2_5b9p0o1h"}},{"node":{"id":"t2_13k4rx"}},{"node":{"id":"t2_mm5icoyd"}},{"node":{"id":"t2_5jinl8hn"}},{"node":{"id":"t2_nqfwv2hp"}},{"node":{"id":"t2_4zcwa"}},{"node":{"id":"t2_bhrdlhdd"}},{"node":{"id":"t2_iu4vpzdh"}},{"node":{"id":"t2_9jd2rzsu"}},{"node":{"id":"t2_ecteky4x"}},{"node":{"id":"t2_6zce890j"}},{"node":{"id":"t2_rqucsbpm"}},{"node":{"id":"t2_70fsihi1"}},{"node":{"id":"t2_ahdcw220"}},{"node":{"id":"t2_26jl29tm"}},{"node":{"id":"t2_4yhpkgcn"}},{"node":{"id":"t2_owfbsdwr"}},{"node":{"id":"t2_337yj"}},{"node":{"id":"t2_6c8wwqu5"}},{"node":{"id":"t2_95hlhzdf"}},{"node":{"id":"t2_kzxa4"}},{"node":{"id":"t2_jnjzydt3"}},{"node":{"id":"t2_9gb7glzb"}},{"node":{"id":"t2_r4fhj"}},{"node":{"id":"t2_1jwvqp93"}},{"node":{"id":"t2_cabvwvj7"}},{"node":{"id":"t2_11z32i"}},{"node":{"id":"t2_1447ny"}},{"node":{"id":"t2_8epsmr9"}},{"node":{"id":"t2_3vouoxu2"}},{"node":{"id":"t2_ixzpt"}},{"node":{"id":"t2_nm8o7lyv"}},{"node":{"id":"t2_dyxp5qqc"}},{"node":{"id":"t2_4uyzj"}},{"node":{"id":"t2_8sns7fw5"}},{"node":{"id":"t2_25tqf6ow"}},{"node":{"id":"t2_16yvsj"}},{"node":{"id":"t2_raxsh9sz"}},{"node":{"id":"t2_i9nkqwwx"}},{"node":{"id":"t2_gexa9ikd"}},{"node":{"id":"t2_ibjs7gne"}},{"node":{"id":"t2_obwlauff"}},{"node":{"id":"t2_a5nn1gpt"}},{"node":{"id":"t2_s4vpb8b1"}},{"node":{"id":"t2_nrrfn"}},{"node":{"id":"t2_f3n27"}},{"node":{"id":"t2_oqm3czw"}},{"node":{"id":"t2_34xy1hgr"}},{"node":{"id":"t2_125cmt30"}},{"node":{"id":"t2_30a2u23k"}},{"node":{"id":"t2_mtsfs"}},{"node":{"id":"t2_unodp"}},{"node":{"id":"t2_joqda"}},{"node":{"id":"t2_hfe33"}},{"node":{"id":"t2_24f2i9dr"}},{"node":{"id":"t2_c8754hab"}},{"node":{"id":"t2_31z5b1ey"}},{"node":{"id":"t2_vgbm98s8"}},{"node":{"id":"t2_7tov03rs"}},{"node":{"id":"t2_2zkhv494"}},{"node":{"id":"t2_javjl"}},{"node":{"id":"t2_jkaojlt4"}},{"node":{"id":"t2_6pr22qbe"}},{"node":{"id":"t2_gvvun"}},{"node":{"id":"t2_2gmtkkis"}},{"node":{"id":"t2_4xs6afd3"}},{"node":{"id":"t2_11fuyzbm"}},{"node":{"id":"t2_nb5p79by"}},{"node":{"id":"t2_ko0nc"}},{"node":{"id":"t2_omrx8jau"}},{"node":{"id":"t2_9wsk6j2t"}},{"node":{"id":"t2_bw4yvfg"}},{"node":{"id":"t2_104slf"}},{"node":{"id":"t2_lerb9"}},{"node":{"id":"t2_yt34o"}},{"node":{"id":"t2_g4ive"}},{"node":{"id":"t2_fe9s7nsa"}},{"node":{"id":"t2_e26iun7u"}},{"node":{"id":"t2_qbn6k4qs"}},{"node":{"id":"t2_ur5m0"}},{"node":{"id":"t2_pv16ul8i"}},{"node":{"id":"t2_8wuk2upu3"}},{"node":{"id":"t2_4neaiiv0"}},{"node":{"id":"t2_s0zqosvb"}},{"node":{"id":"t2_a6os1ahio"}},{"node":{"id":"t2_0"}},{"node":{"id":"t2_dhk5kun1"}},{"node":{"id":"t2_ahw8bo31"}},{"node":{"id":"t2_15dcxe"}},{"node":{"id":"t2_g5sek"}},{"node":{"id":"t2_qa2ooynd"}},{"node":{"id":"t2_bnc4g"}},{"node":{"id":"t2_9eqwelnl"}},{"node":{"id":"t2_88cvndv1"}},{"node":{"id":"t2_w39ugh7u"}},{"node":{"id":"t2_hnyl0"}},{"node":{"id":"t2_5zt9vb94"}},{"node":{"id":"t2_6esk2"}},{"node":{"id":"t2_8m1smhqg"}},{"node":{"id":"t2_u3gzljfz"}},{"node":{"id":"t2_fem71br"}}]}}}}


use actix_web::{HttpRequest, HttpResponse, ResponseError};
use lemmy_client::{lemmy_api_common::LemmyErrorType, ClientOptions, LemmyClient, LemmyRequest};
use serde_json::json;
use GetBlockedUsers::*;

use crate::{get_jwt, HackTraitPerson};

#[derive(Debug)]
pub enum GetBlockedUsers {
    Authentication,
    GetSite(LemmyErrorType),
    MissingUser,
}

impl std::fmt::Display for GetBlockedUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authentication => write!(f, "Authentication error"),
            GetSite(e) => write!(f, "Get site error: {}", e),
            MissingUser => write!(f, "Missing user"),
        }
    }
}

impl ResponseError for GetBlockedUsers {
    fn status_code(&self) -> awc::http::StatusCode {
        eprintln!("{self}");
        awc::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_blocked_users(request: HttpRequest) -> Result<HttpResponse, GetBlockedUsers> {
    let jwt = get_jwt(&request).ok_or(Authentication)?;

    let client = LemmyClient::new(ClientOptions {
        domain: String::from("jlai.lu"),
        secure: true
    });

    let site = client.get_site(LemmyRequest { body: (), jwt: Some(jwt.clone()) }).await.map_err(GetSite)?;
    let my_user = site.my_user.ok_or(MissingUser)?;

    let rep = json! {{
        "data": {
            "identity": {
                "blockedRedditorsInfo": {
                    "pageInfo": {
                        "__typename": "PageInfo",
                        "hasNextPage": false,
                        "endCursor": "dDJfZmVtNzFicg=="
                    },
                    "edges": my_user.person_blocks.iter().map(|person_block| json! {{
                        "node": {
                            "id": person_block.target.reddit_id()
                        }
                    }}).collect::<Vec<_>>()
                }
            }
        }
    }};

    Ok(HttpResponse::Ok().json(rep))
}
