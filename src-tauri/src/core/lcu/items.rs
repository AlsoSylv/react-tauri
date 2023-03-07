use irelia::rest;
use serde_json::Value;

use crate::errors::LCUResponses;

/// Attempts to push an item set to the client via the LCU API
///
/// Requires JSON as an argument
pub async fn push_items_to_client(page: Value) -> Result<LCUResponses, LCUResponses> {
    if let Ok(client) = rest::LCUClient::new() {
        if let Ok(summoner) = client
            .get::<Value>("/lol-summoner/v1/current-summoner")
            .await
        {
            let item_pages_request = format!(
                "/lol-item-sets/v1/item-sets/{}/sets",
                summoner.unwrap()["summonerId"]
            );
            if let Ok(json) = client.get::<Value>(&item_pages_request).await {
                let mut pages = json.clone().unwrap();
                pages["itemSets"].as_array_mut().unwrap().push(page);
                // This will always return an EOF error, so the value is useless
                let _ = client.put::<Value, Value>(&item_pages_request, pages).await;
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
