use crate::{DataDragon, DataDragonError, request};
use serde_json::Value;

impl DataDragon {
    pub async fn item_json(&self) -> Result<Value, DataDragonError> {
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/item.json",
            &self.version, &self.language
        );
        let item_json = request::<Value, DataDragonError>(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        Ok(item_json)
    }
}