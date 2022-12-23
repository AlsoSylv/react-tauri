use serde_json::Value;
use shaco::rest;

use crate::errors::LCUResponses;

/// Attempts to push an item set to the client via the LCU API
/// this will eventually end up wrapped in some form of struct
/// that handles checking if the LCU is open
/// 
/// Requires JSON as an argument
pub async fn push_items_to_client(page: Value) -> Result<LCUResponses, LCUResponses> {
    if let Ok(client) = rest::RESTClient::new() {
        if let Ok(summoner) = client
            .get("/lol-summoner/v1/current-summoner".to_owned())
            .await {
                let item_pages_request = format!("/lol-item-sets/v1/item-sets/{}/sets", summoner["summonerId"]);
                if let Ok(json) = client.get(item_pages_request.clone()).await {
                    let mut pages = json.clone();
                    pages["itemSets"].as_array_mut().unwrap().push(page);
                    // This will always return an EOF error, so the value is useless
                    let _ = client.put(item_pages_request, pages).await;          
                    Ok(LCUResponses::LCUItemsPushedMaybe)
                } else {
                    Err(LCUResponses::LCUGetItems)
                }
            } else {
                Err(LCUResponses::LCUGetSummoner)
            }
    } else {
        Err(LCUResponses::LCUConnect)
    }
}
