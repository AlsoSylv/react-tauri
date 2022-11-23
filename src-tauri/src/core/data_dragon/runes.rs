use super::structs::{self, RuneTree};

use crate::errors::DataDragonError;
use moka::future::{Cache, ConcurrentCacheExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static CACHED_RUNE_JSON: Lazy<Mutex<Cache<String, Vec<RuneTree>>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl structs::DataDragon {
    /// A chached function to get `runesReforged.json` from data dragon
    ///
    /// # Example
    /// ```
    /// let data_dragon = DataDragon::new(None).await.unwrap();
    /// let champion_json = data_dragon.runes_json("Xayah".to_owned()).await;
    /// ```
    pub async fn runes_json(&self) -> Result<Vec<RuneTree>, DataDragonError> {
        let cache = CACHED_RUNE_JSON.lock().await;
        if let Some(json) = cache.get(&self.language) {
            return Ok(json);
        };

        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/runesReforged.json",
            &self.version, &self.language
        );
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let Ok(rune_json) = response.json::<Vec<RuneTree>>().await else {
                    return Err(DataDragonError::DataDragonMissing);
                };
                cache.insert(self.language.clone(), rune_json.clone()).await;
                cache.sync();
                Ok(rune_json)
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
}
