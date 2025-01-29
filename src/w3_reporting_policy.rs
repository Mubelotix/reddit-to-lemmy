// ClientRequest HTTP/1.1 GET https://w3-reporting.reddit.com/policy
//   headers:
//     "accept-encoding": "gzip"
//     "user-agent": "okhttp/5.0.0-alpha.14"

// binary
// HttpResponse { error: None, res: 
// Response HTTP/1.1 204 No Content
//   headers:
//     "accept-ranges": "bytes"
//     "cache-control": "max-age=14400"
//     "access-control-expose-headers": "*"
//     "access-control-allow-methods": "POST, OPTIONS"
//     "via": "1.1 varnish"
//     "access-control-max-age": "86400"
//     "vary": "Origin"
//     "x-content-type-options": "nosniff"
//     "x-xss-protection": "1; mode=block"
//     "content-length": "0"
//     "date": "Wed, 29 Jan 2025 23:18:33 GMT"
//     "retry-after": "0"
//     "access-control-allow-headers": "Content-Type,Origin,X-origination-host,X-origination-path"
//     "access-control-allow-origin": "https://www.reddit.com"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "x-frame-options": "SAMEORIGIN"
//     "x-reddit-w3reporting": "{\"report_to\": \"w3-reporting\", \"max_age\": 14400, \"success_fraction\": 0.99, \"failure_fraction\": 0.99}"
//     "server": "Varnish"
//     "report-to": "{\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}"
//   body: Sized(0)
//  }
// binary dat

use actix_web::{get, HttpResponse};
use log::debug;

#[get("/w3-reporting.reddit.com/policy")]
pub async fn w3_reporting_policy() -> HttpResponse {
    debug!("w3_reporting_policy");

    HttpResponse::NoContent()
        .append_header(("accept-ranges", "bytes"))
        .append_header(("cache-control", "max-age=14400"))
        .append_header(("access-control-expose-headers", "*"))
        .append_header(("access-control-allow-methods", "POST, OPTIONS"))
        .append_header(("via", "1.1 varnish"))
        .append_header(("access-control-max-age", "86400"))
        .append_header(("vary", "Origin"))
        .append_header(("x-content-type-options", "nosniff"))
        .append_header(("x-xss-protection", "1; mode=block"))
        .append_header(("content-length", "0"))
        .append_header(("date", "Wed, 29 Jan 2025 23:18:33 GMT"))
        .append_header(("retry-after", "0"))
        .append_header(("access-control-allow-headers", "Content-Type,Origin,X-origination-host,X-origination-path"))
        .append_header(("access-control-allow-origin", "https://www.reddit.com"))
        .append_header(("strict-transport-security", "max-age=31536000; includeSubdomains"))
        .append_header(("x-frame-options", "SAMEORIGIN"))
        .append_header(("x-reddit-w3reporting", "{\"report_to\": \"w3-reporting\", \"max_age\": 14400, \"success_fraction\": 0.99, \"failure_fraction\": 0.99}"))
        .append_header(("server", "Varnish"))
        .append_header(("report-to", "{\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}"))
        .finish()
}
