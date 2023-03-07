use serde::{Deserialize, Serialize};

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
    match client.get(url).send().await {
        Ok(response) => response.json::<T>().await.map_err(|_| error_one),
        Err(err) => {
            if err.is_body() {
                Err(error_one)
            } else {
                Err(error_two)
            }
        }
    }
}

/// Temple for GraphQL requests that uses generics
/// And Handles deserialization of the response body
/// from JSON to a Rust Struct
///
/// # Example
/// ```rs
/// use crate::templates::gql_request;
/// use crate::errors;
///
/// async fn new_gql_request(&self) {
///     let request = gql_request::<Value, Value, CommunityDragonError>(
///         "<query>",
///         "<vars>",
///         &self.client,
///         "<URL>",
///         errors::CommunityDragonMissing,
///         errors::CannotConnect,
///     ).await
///     match request {
///         Ok(json) => { ... }
///         Err(err) => { ... }
///     }
/// }
async fn gql_request<Vars: Serialize, Data: for<'de> Deserialize<'de>, E>(
    query: &str,
    vars: Vars,
    client: &reqwest::Client,
    url: &str,
    error_one: E,
    error_two: E,
) -> Result<Data, E> {
    match client
        .post(url)
        .json(&GQLQuery {
            variables: vars,
            query: query,
        })
        .send()
        .await
    {
        Ok(response) => response.json::<Data>().await.map_err(|_| error_one),
        Err(err) => {
            if err.is_body() {
                Err(error_one)
            } else {
                Err(error_two)
            }
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GQLQuery<'a, T> {
    variables: T,
    query: &'a str,
}
