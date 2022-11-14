use serde_json::Value;

use super::structs;

use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use moka::future::{Cache, ConcurrentCacheExt};

static CACHED_ITEM_JSON: Lazy<Mutex<Cache<String, Value>>> = Lazy::new(|| {
    Mutex::new(Cache::new(3))
});


impl structs::DataDragon {
    pub async fn item_json(&self) -> Result<Value, i64> {
        let cache = CACHED_ITEM_JSON.lock().await;
        let json = cache.get(&self.language);
        if json.is_some() {
            return Ok(json.unwrap());
        }

        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/item.json",
            &self.version,
            &self.language
        );
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let item_json: Result<Value, reqwest::Error> = response.json().await;
                match item_json {
                    Ok(item_json) => {
                        cache.insert(self.language.clone(), item_json.clone()).await;
                        cache.sync();
                        Ok(item_json)
                    },
                    Err(_) => Err(103),
                }
            },
            Err(err) => {
                if err.is_body() {
                    Err(104)
                } else {
                    Err(103)
                }
            }
        }
    }
}