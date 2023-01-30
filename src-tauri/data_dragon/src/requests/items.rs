use crate::{request, DataDragon, DataDragonError};
use serde_json::Value;

use moka::future::{Cache, ConcurrentCacheExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static CACHED_ITEM_JSON: Lazy<Mutex<Cache<String, Value>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl DataDragon {
    /// Cached function to get Data Dragons items.json file
    ///
    /// TODO: Return as a struct, not Value
    ///
    /// # Example
    /// ```rust
    /// async fn items_test() {
    ///     use data_dragon::DataDragon;
    ///
    ///     let data_dragon = DataDragon::new(None).await;
    ///     match data_dragon {
    ///         Ok(data_dragon) => {
    ///             let items = data_dragon.item_json().await;
    ///             match items {
    ///                 Ok(json) => {
    ///                     if let Some(boots) = json["data"]["1001"]["name"].as_str() {
    ///                         assert!(boots == "Boots");
    ///                     } else {
    ///                         panic!()
    ///                     }
    ///                 }
    ///                 Err(_) => panic!(),
    ///             }
    ///         }
    ///         Err(_) => panic!(),
    ///     }
    /// }
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
        let item_json: Value = request(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        cache.insert(self.language.clone(), item_json.clone()).await;
        cache.sync();
        Ok(item_json)
    }
}
