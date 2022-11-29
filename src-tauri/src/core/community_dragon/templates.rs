use serde::Deserialize;
use crate::errors::CommunityDragonError;

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
