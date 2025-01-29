// No idea what this is but it's easy to fake.
//
// ClientRequest HTTP/1.1 POST https://e.reddit.com/v2c
//   headers:
//     "content-length": "2666"
//     "accept-encoding": "gzip"
//     "x-reddit-qos": "down-rate-mbps=1.700"
//     "x-signature": "key=RedditAppAndroid-v2, mac=5f2b5487a44288f57275aad7ba2b78679936ec71cc7a298d4aebccd563e58dec"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "date": "Thu Jan 30 00:18:36 GMT+01:00 2025"
//     "x-reddit-compression": "1"
//     "content-type": "application/octet-stream"
//
// binary
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "content-length": "0"
//     "accept-ranges": "bytes"
//     "date": "Wed, 29 Jan 2025 23:18:37 GMT"
//     "x-content-type-options": "nosniff"
//     "x-xss-protection": "1; mode=block"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "content-type": "text/plain; charset=utf-8"
//     "x-frame-options": "SAMEORIGIN"
//     "via": "1.1 varnish"
//     "vary": "Origin"
//   body: Sized(0)
//  }

use actix_web::{get, HttpResponse};
use log::debug;

#[get("/e.reddit.com/v2c")]
pub async fn v2c() -> HttpResponse {
    debug!("v2c");

    HttpResponse::Ok()
        .append_header(("content-length", "0"))
        .append_header(("accept-ranges", "bytes"))
        .append_header(("date", "Wed, 29 Jan 2025 23:18:37 GMT"))
        .append_header(("x-content-type-options", "nosniff"))
        .append_header(("x-xss-protection", "1; mode=block"))
        .append_header(("strict-transport-security", "max-age=31536000; includeSubdomains"))
        .append_header(("content-type", "text/plain; charset=utf-8"))
        .append_header(("x-frame-options", "SAMEORIGIN"))
        .append_header(("via", "1.1 varnish"))
        .append_header(("vary", "Origin"))
        .finish()
}
