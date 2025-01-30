// {"operationName":"EmailPermission","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"19c21f024590fd4026f3c6adba43b81dd2b5dc46d0a51bf4443d8c69bc5d9625"}}}
// 
// {"data":{"identity":null}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_email_permission() -> HttpResponse {
    debug!("get_email_permission");

    let rep = json! {{
        "data": {
            "identity": null
        }
    }};

    HttpResponse::Ok().json(rep)
}
