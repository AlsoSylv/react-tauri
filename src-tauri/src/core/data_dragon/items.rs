use serde_json::Value;

use super::structs;

use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use moka::future::{Cache, ConcurrentCacheExt};
use crate::errors::DataDragonError;

static CACHED_ITEM_JSON: Lazy<Mutex<Cache<String, Value>>> = Lazy::new(|| {
    Mutex::new(Cache::new(3))
});


impl structs::DataDragon {
    pub async fn item_json(&self) -> Result<Value, DataDragonError> {
        let cache = CACHED_ITEM_JSON.lock().await;
        if let Some(json) = cache.get(&self.language) {
            return Ok(json);
        };

        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/item.json",
            &self.version,
            &self.language
        );
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let Ok(item_json) = response.json::<Value>().await else {
                    return Err(DataDragonError::CannotConnect);
                };
                cache.insert(self.language.clone(), item_json.clone()).await;
                cache.sync();
                Ok(item_json)
            },
            Err(err) => {
                if err.is_body() {
                    Err(DataDragonError::DataDragonMissing)
                } else {
                    Err(DataDragonError::CannotConnect)
                }
            }
        }
    }
}