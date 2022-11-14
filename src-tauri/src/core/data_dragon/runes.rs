use super::structs::{self, RuneTree};

use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use moka::sync::{Cache, ConcurrentCacheExt};

static CACHED_RUNE_JSON: Lazy<Mutex<Cache<String, Vec<RuneTree>>>> = Lazy::new(|| {
    Mutex::new(Cache::new(3))
});


impl structs::DataDragon {
    pub async fn runes_json(&self) -> Result<Vec<RuneTree>, i64> {
        let cache = CACHED_RUNE_JSON.lock().await;
        let json = cache.get(&self.language);
        if json.is_some() {
            return Ok(json.unwrap());
        }
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/runesReforged.json", 
            &self.version,
            &self.language
        );
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let rune_json: Result<Vec<RuneTree>, reqwest::Error> = response.json().await;
                match rune_json {
                    Ok(rune_json) => {
                        cache.insert(self.language.clone(), rune_json.clone());
                        cache.sync();
                        Ok(rune_json)
                    },
                    Err(_) => Err(104),
                }
            }
            Err(_) => Err(104),
        }
    }
}