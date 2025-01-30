// {"operationName":"AllDynamicConfigs","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"a4056084f78a61e765d0c5b654a8ccbff4923732fc69c9ee907a57ef4c1bb2ce"}}}
//
// see config/dynamic_configs.json

use actix_web::HttpResponse;
use log::debug;

pub async fn get_dynamic_configs() -> HttpResponse {
    debug!("get_dynamic_configs");

    let rep = include_bytes!("config/dynamic_configs.json");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(rep.to_vec())
}
