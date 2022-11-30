use crate::errors::CommunityDragonError;
use serde::Deserialize;

/// Template for requests that uses generics
/// Handles deserializing the body of the
/// request from to a struct from JSON
///
/// # Example
/// ```rs
/// use super::templates::request;
/// use super::structs;
///
/// impl structs::CommunityDragon {
///     async fn new_request(&self) {
///         let request: Result<Value, CommunityDragonError> = request::<Value>("<URL>", &self.client).await;
///         match request {
///             Ok(json) => { ... },
///             Err(err) => { ... },
///         }
///     }
/// }
/// ```
pub async fn request<T: for<'de> Deserialize<'de>>(
    url: String,
    client: &reqwest::Client,
) -> Result<T, CommunityDragonError> {
    let request = client.get(url).send().await;
    match request {
        Ok(response) => {
            let Ok(json) = response.json::<T>().await else {
                todo!()
            };
            Ok(json)
        }
        Err(err) => {
            if err.is_body() {
                todo!()
            } else {
                todo!()
            }
        }
    }
}
