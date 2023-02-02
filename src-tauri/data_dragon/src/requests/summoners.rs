use crate::{request, types::Summoners, DataDragon, DataDragonError};

use moka::future::{Cache, ConcurrentCacheExt};
use tokio::sync::Mutex;

use once_cell::sync::Lazy;

static CACHED_SUMMONERS_JSON: Lazy<Mutex<Cache<String, Summoners>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl DataDragon {
    pub async fn summoners_json(
        &self,
        version: &str,
        language: Option<&str>,
    ) -> Result<Summoners, DataDragonError> {
        let lang = self.lang_default(language);
        let cache = CACHED_SUMMONERS_JSON.lock().await;
        if let Some(json) = cache.get(lang) {
            return Ok(json);
        };
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/runesReforged.json",
            version, lang
        );
        let summoner_json: Summoners = request(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        cache.insert(lang.to_string(), summoner_json.clone()).await;
        cache.sync();
        Ok(summoner_json)
    }
}
