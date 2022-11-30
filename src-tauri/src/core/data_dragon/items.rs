use serde_json::Value;

use super::structs;

use crate::templates::request;

use super::DataDragon;

use crate::errors::DataDragonError;
use moka::future::{Cache, ConcurrentCacheExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static CACHED_ITEM_JSON: Lazy<Mutex<Cache<String, Value>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl DataDragon {
    /// A cached function to get `item.json` from Data Dragon
    ///
    /// # Example
    /// ```
    /// let data_dragon = DataDragon::new(None).await.unwrap();
    /// let champion_json = data_dragon.item_json("Xayah".to_owned()).await;
    /// ```
    pub async fn item_json(&self) -> Result<Value, DataDragonError> {
        let cache = CACHED_ITEM_JSON.lock().await;
        if let Some(json) = cache.get(&self.language) {
            return Ok(json);
        };

        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/item.json",
            &self.version, &self.language
        );
        let request = request::<Value, DataDragonError>(url.to_owned(), &self.client, DataDragonError::DataDragonMissing, DataDragonError::CannotConnect).await;
        match request {
            Ok(item_json) => {
                cache.insert(self.language.clone(), item_json.clone()).await;
                cache.sync();
                Ok(item_json)
            }
            Err(err) => Err(err),
        }
    }
}
