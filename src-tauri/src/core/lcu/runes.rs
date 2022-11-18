use shaco::rest;
use serde_json::Value;

use crate::errors::LCUResponses;

pub async fn push_runes_to_client(page: Value) -> Result<LCUResponses, LCUResponses> {
    let client = rest::RESTClient::new();
    let pages_endpoint = String::from("/lol-perks/v1/pages");
    match client {
        Ok(client) => {
            let post = client.put(pages_endpoint.clone(), page.clone()).await;
            match post {
                Ok(_) => Ok(LCUResponses::LCUPushRune),
                Err(_) => {
                    let response = client.get("/lol-perks/v1/currentpage".to_string()).await;
                    match response {
                        Ok(response) => {
                            let Some(id) = &response["id"].as_i64() else {
                                panic!();
                            };
                            let delete = client.delete(format!("{0}/{1}", pages_endpoint, id)).await;
                            match delete {
                                Ok(_) => {
                                    let post = client.put(pages_endpoint, page).await;
                                    match post {
                                        Ok(_) => Ok(LCUResponses::LCUPushRune),
                                        Err(_) => Err(LCUResponses::LCUCreateRune),
                                    }
                                }
                                Err(_) => Err(LCUResponses::LCUDeleteRune)
                            }
                        },
                        Err(_) => Err(LCUResponses::LCUGetRune)
                    }
                },
            }
        },
        Err(_) => Err(LCUResponses::LCUConnect)
    }
}