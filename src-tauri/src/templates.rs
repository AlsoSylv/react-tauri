use serde::Deserialize;

/// Template for requests that uses generics
/// Handles deserializing the body of the
/// request from to a struct from JSON
///
/// # Example
/// ```rs
/// use crate::templates::request;
/// use crate::errors;
///
/// 
/// async fn new_request(&self) {
///     let request: Result<Value, CommunityDragonError> = request::<Value, CommunityDragonError>(
///         "<URL>", 
///         &self.client,
///         errors::CommunityDragonMissing,
///         errors::CannotConnect,
///         ).await;
///     match request {
///         Ok(json) => { ... },
///         Err(err) => { ... },
///     }
/// }
/// ```
pub async fn request<T: for<'de> Deserialize<'de>, E>(
    url: String,
    client: &reqwest::Client,
    error_one: E,
    error_two: E,
) -> Result<T, E> {
    let request = client.get(url).send().await;
    match request {
        Ok(response) => {
            let Ok(json) = response.json::<T>().await else {
                return Err(error_one);
            };
            Ok(json)
        }
        Err(err) => {
            if err.is_body() {
                Err(error_one)
            } else {
                Err(error_two)
            }
        }
    }
}