// ClientRequest HTTP/1.1 POST https://e.reddit.com/v2p
//   headers:
//     "date": "Thu Jan 30 00:18:39 GMT+01:00 2025"
//     "content-type": "application/octet-stream"
//     "accept-encoding": "gzip"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "cookie": "code-server-session=%24argon2id%24v%3D19%24m%3D65536%2Ct%3D3%2Cp%3D4%240drmhGesFIn1P7VzGSBrHw%24FxCv8Zksh36rgyKhqNI5wwp3uRuLWyjtTlFF80QLacc"
//     "x-signature": "key=RedditAppAndroidProto-v1, mac=54394beeec46b5793a55347776cd72b805e3de6e3f462de86a10858b8a058bd3"
//     "x-reddit-qos": "down-rate-mbps=1.700"
//     "x-reddit-compression": "1"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "content-length": "255"
// 
// binary
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "vary": "Origin"
//     "x-frame-options": "SAMEORIGIN"
//     "x-xss-protection": "1; mode=block"
//     "content-length": "0"
//     "content-type": "text/plain; charset=utf-8"
//     "x-content-type-options": "nosniff"
//     "date": "Wed, 29 Jan 2025 23:18:39 GMT"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "accept-ranges": "bytes"
//     "via": "1.1 varnish"
//   body: Sized(0)
//  }

use actix_web::{post, HttpResponse};
use log::debug;

#[post("/e.reddit.com/v2p")]
pub async fn v2p() -> HttpResponse {
    debug!("v2p");

    HttpResponse::Ok()
        .append_header(("content-length", "0"))
        .append_header(("accept-ranges", "bytes"))
        .append_header(("date", "Wed, 29 Jan 2025 23:18:39 GMT"))
        .append_header(("x-content-type-options", "nosniff"))
        .append_header(("x-xss-protection", "1; mode=block"))
        .append_header(("strict-transport-security", "max-age=31536000; includeSubdomains"))
        .append_header(("content-type", "text/plain; charset=utf-8"))
        .append_header(("x-frame-options", "SAMEORIGIN"))
        .append_header(("via", "1.1 varnish"))
        .append_header(("vary", "Origin"))
        .finish()
}
