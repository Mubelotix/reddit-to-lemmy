// {"operationName":"UserLocation","variables":{},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"f07de258c54537e24d7856080f662c1b1268210251e5789c8c08f20d76cc8ab2"}}}
// 
// {"data":{"userLocation":{"countryCode":"FR","regionCode":"NOR","cityCode":"NOTRE-DAME-DE-BONDEVILLE","cityUtf8":"NOTRE-DAME-DE-BONDEVILLE"}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_location() -> HttpResponse {
    debug!("get_location");

    let rep = json! {{
        "data": {
            "userLocation": {
                "countryCode": "US",
                "regionCode": "CA",
                "cityCode": "LOS-ANGELES",
                "cityUtf8": "LOS-ANGELES"
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
