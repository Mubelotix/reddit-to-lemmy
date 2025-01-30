// {"operationName":"GetInventoryItemsByIds","variables":{"ids":["nft_eip155:137_c8d3a3a83bde5dad06d436694e3e22ac3e64d577_3906012"]},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"b8d35af133fcfe112baec9c898333f7cf217ede2a467ab60467a6f839c572263"}}}
// 
// {"data":{"inventoryItems":{"edges":[{"node":{"__typename":"InventoryItem","id":"nft_eip155:137_c8d3a3a83bde5dad06d436694e3e22ac3e64d577_3906012","name":"Pixel Placers #53787","tags":["MINTED"],"serialNumber":"53787","owner":{"id":"t2_5xrogxaw","displayName":"Mubelotix"},"artist":null,"benefits":{"avatarOutfit":{"id":"01GK5KTRN8RFZT3W675AJFNWN2","preRenderImage":{"url":"https://i.redd.it/snoovatar/avatars/basic/9d0d13cc-e7aa-4678-8459-e2b073efe731.png"},"backgroundImage":{"url":"https://i.redd.it/snoovatar/submission_snoo_assets/_Ly7aSs63Pk_.png"}},"avatarUtilities":[]},"drop":{"size":null,"rarity":null},"nft":{"contractAddress":"c8d3a3a83bde5dad06d436694e3e22ac3e64d577","title":"Pixel Placers #53787","description":"Redditors love a blank canvas.","externalUrls":["https://polygonscan.com/token/0xc8d3a3a83bde5dad06d436694e3e22ac3e64d577?a=3906012"],"series":"Pixel Placers","mintedAt":"2022-12-17T15:31:24.000000+0000","tokenUrl":"https://reddit.infura-ipfs.io/ipfs/QmPWUhx4rH2gZvkwuqTTNt1H11hE2ySC1yJ2Gv5QvDkXqn/3906012.json","tokenId":"3906012","imageUrl":"https://reddit.infura-ipfs.io/ipfs/QmPgxQF7fYaHPoxd5Njz57qgGwnwJVNo4Vc4cvQp63yX31","wallet":{"address":"17463b2d658c7019b46d0c21aab05d44e7313770"}}}}]}}}

use actix_web::HttpResponse;
use serde_json::json;
use log::debug;

pub async fn get_inventory_items() -> HttpResponse {
    debug!("get_inventory_items");

    let rep = json! {{
        "data": {
            "inventoryItems": {
                "edges": []
            }
        }
    }};

    HttpResponse::Ok().json(rep)
}
