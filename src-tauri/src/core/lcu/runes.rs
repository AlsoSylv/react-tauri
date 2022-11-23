use serde_json::Value;
use shaco::rest;

use crate::errors::LCUResponses;

/// Attempts to push runes to the LoL Client Via the LCU API
/// this will eventually end up wrapped in some sort of struct
/// that handles checking if the LCU exists
///
/// Requires JSON as an argument
pub async fn push_runes_to_client(page: Value) -> Result<LCUResponses, LCUResponses> {
    let pages_endpoint = String::from("/lol-perks/v1/pages");
    if let Ok(client) = rest::RESTClient::new() {
        if client
            .put(pages_endpoint.clone(), page.clone())
            .await
            .is_ok()
        {
            Ok(LCUResponses::LCUPushRune)
        } else {
            if let Ok(response) = client.get("/lol-perks/v1/currentpage".to_string()).await {
                let Some(id) = &response["id"].as_i64() else {
                    panic!();
                };
                if client
                    .delete(format!("{0}/{1}", pages_endpoint, id))
                    .await
                    .is_ok()
                {
                    if client.put(pages_endpoint, page).await.is_ok() {
                        Ok(LCUResponses::LCUPushRune)
                    } else {
                        Err(LCUResponses::LCUCreateRune)
                    }
                } else {
                    Err(LCUResponses::LCUDeleteRune)
                }
            } else {
                Err(LCUResponses::LCUGetRune)
            }
        }
    } else {
        Err(LCUResponses::LCUConnect)
    }
}
