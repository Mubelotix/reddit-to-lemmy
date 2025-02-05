// ClientRequest HTTP/1.1 POST https://www.reddit.com/auth/v2/login/password
//   headers:
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "x-hmac-signed-result": "1:android:2:1738077743:0ad910da15ea45749597e7620e6ec36dc810e70e3541685e66a2a11688d343f1"
//     "x-reddit-compression": "1"
//     "content-length": "3411"
//     "content-type": "application/json; charset=UTF-8"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "x-reddit-qos": "down-rate-mbps=1.700"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-hmac-signed-body": "1:android:2:1738077743:9add1fab259eb4a34f60d1c6e3cf4edcd36e8465681c5f771ffe18ec5650051d"
//     "accept-encoding": "gzip"
//     "x-reddit-loid": "00000000005xrogxaw.2.1584284301614.Z0FBQUFBQm5sNkxJcnEyMEJJYmdBc2k0MHd6TTk1c204RlZlaUxUbmFMeTlaV09MaWdIWW9nN0h3Y1RzUE5tc21iSXNsMms1ZDREQmRMTWFSam5zUW1JUVE0ZE4xU2tLMnItTzZzcUxRSUNRMVUyWFBwQ2g2YmtIT3FLNmI1dUVsa3d2Yk1RamRqb0c"
// 
// "{\"identifier\":\"mubelotix@gmail.com\",\"password\":\"redacted\",\"recaptcha_token\":\"redacted\",\"app_name\":\"android\"}"
// 
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "x-reddit-session": "oqmeebimbrmalgrnjp.0.1738077747166.Z0FBQUFBQm5tUFl6T1NZcHBCRHNvWkYtMVlfUUtQbWwtMG9WYVFvNzhGbEt1UUk2dV9OQVJnZmp1VWJfYTloY2xuRjIzRk5Qd3E3SDVkOERpSGsyOTQ4N1gtUUlWU3BtQVlWclRwVHRzWGtqR1ZJcVUyVFpYUjg3RVlrcFVtLUtwd0FicDZNdFFQSno"
//     "x-frame-options": "SAMEORIGIN"
//     "accept-ranges": "bytes"
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 1.0, \"failure_fraction\": 1.0}"
//     "x-content-type-options": "nosniff"
//     "x-xss-protection": "1; mode=block"
//     "set-cookie": "reddit_session=redacted_jwt_token; Path=/; Domain=reddit.com; Max-Age=15638399; HttpOnly; Secure"
//     "set-cookie": "edgebucket=2uKIDtej26JFnN9C2T; Domain=reddit.com; Max-Age=63071999; Path=/;  secure"
//     "server": "snooserv"
//     "content-length": "2"
//     "via": "1.1 varnish"
//     "cache-control": "private, max-age=3600"
//     "x-reddit-loid": "00000000005xrogxaw.2.1584284301614.Z0FBQUFBQm5tUFl6UnEzN1BjYllmc1hiam5fTUdRZHE5V0hLTXgtc3ZYNlNqSzNNZUY5aGk3VjBMd2ZCT0xXd0U4TG50Rjk0VUlEUkZiRXBJSWtXbVNKaFNXRHlDaldXWkFDLU9zdXAwcFFOSDF0OEdXT3RVZ3lWNmVGS0ZuZ1JERjZ4Z1pERG5yWkM"
//     "date": "Tue, 28 Jan 2025 15:22:27 GMT"
//     "content-type": "application/json"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//   body: Sized(2)
//  }
// "{}"

use actix_web::{post, web::Json, HttpResponse, Responder};
use lemmy_client::{ClientOptions, LemmyClient};
use serde::Deserialize;
use log::{debug, trace};

#[derive(Debug, Deserialize)]
struct LoginPassword {
    identifier: String,
    password: String,
    recaptcha_token: String,
    app_name: String,
}

#[post("/www.reddit.com/auth/v2/login/password")]
async fn login(login: Json<LoginPassword>) -> impl Responder {
    debug!("login {login:?}");

    let (user, instance) = login.identifier.split_once('@').unwrap();

    let client = LemmyClient::new(ClientOptions {
        domain: String::from(instance),
        secure: true
    });

    let response = client.login(lemmy_client::lemmy_api_common::person::Login {
        username_or_email: user.to_string().into(),
        password: login.password.clone().into(),
        totp_2fa_token: None,
    }).await.unwrap();

    let jwt = response.jwt.unwrap().into_inner();

    trace!("login success");
    HttpResponse::Ok()
        .append_header(("set-cookie", format!("reddit_session={jwt}; Path=/; Domain=reddit.com; Max-Age=15638399; HttpOnly; Secure")))
        .json(())
}
