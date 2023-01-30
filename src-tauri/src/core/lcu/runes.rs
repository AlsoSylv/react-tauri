use irelia::rest;
use serde_json::Value;

use crate::errors::LCUResponses;

/// Attempts to push runes to the LoL Client Via the LCU API
///
/// Requires JSON as an argument
pub async fn push_runes_to_client(page: Value) -> Result<LCUResponses, LCUResponses> {
    let pages_endpoint = "/lol-perks/v1/pages";
    if let Ok(client) = rest::LCUClient::new() {
        if let Ok(response) = client.get::<Value>("/lol-perks/v1/currentpage").await {
            let response = response.unwrap();
            let Some(id) = &response["id"].as_i64() else {
                panic!();
            };
            if client
                .delete::<Value>(&format!("/lol-perks/v1/page/{}", id))
                .await
                .is_ok()
            {
                if client
                    .post::<Value, Value>(&pages_endpoint, page)
                    .await
                    .is_ok()
                {
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
    } else {
        Err(LCUResponses::LCUConnect)
    }
}
