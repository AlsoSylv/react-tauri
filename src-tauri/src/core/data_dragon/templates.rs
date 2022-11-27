use crate::errors::DataDragonError;
use serde::Deserialize;

pub async fn request<T: for<'de> Deserialize<'de>>(
    url: &str,
    client: &reqwest::Client,
) -> Result<T, DataDragonError> {
    let request = client.get(url).send().await;
    match request {
        Ok(response) => {
            if let Ok(json) = response.json::<T>().await {
                Ok(json)
            } else {
                Err(DataDragonError::DataDragonMissing)
            }
        }
        Err(err) => {
            if err.is_body() {
                Err(DataDragonError::DataDragonMissing)
            } else {
                Err(DataDragonError::CannotConnect)
            }
        }
    }
}
