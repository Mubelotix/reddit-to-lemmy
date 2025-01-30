// {"operationName":"UsernameAndExperiments","variables":{"inputs":[{"field":"device_id","value":"fcf6b149-1386-458f-9379-04cd6e3d0460"},{"field":"app_version","value":"2025.03.1.2181077"},{"field":"build_number","value":"2181077"}]},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"9bd622448d88f4232245ffdf82943a8ad22727e8d93a7666ac907cac077fd88a"}}}
//
// see config/experiments.json

use actix_web::HttpResponse;
use log::debug;

// Note: The endpoint should return the username but let's try to just return the experiments for now

pub async fn get_experiments() -> HttpResponse {
    debug!("get_experiments");

    let rep = include_bytes!("config/experiments.json");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(rep.to_vec())
}
