use crate::{request, types::RuneTree, DataDragon, DataDragonError};

use moka::future::{Cache, ConcurrentCacheExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static CACHED_RUNE_JSON: Lazy<Mutex<Cache<String, Vec<RuneTree>>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl DataDragon {
    pub async fn rune_json(&self) -> Result<Vec<RuneTree>, DataDragonError> {
        let cache = CACHED_RUNE_JSON.lock().await;
        if let Some(json) = cache.get(&self.language) {
            return Ok(json);
        };
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/runesReforged.json",
            &self.version, &self.language
        );
        let rune_json = request::<Vec<RuneTree>, DataDragonError>(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        cache.insert(self.language.clone(), rune_json.clone()).await;
        cache.sync();
        Ok(rune_json)
    }
}
