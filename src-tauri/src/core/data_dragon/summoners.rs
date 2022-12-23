use moka::future::{Cache, ConcurrentCacheExt};
use tokio::sync::Mutex;

use once_cell::sync::Lazy;

use crate::{errors::DataDragonError, templates::request};

use super::{DataDragon, structs::Summoners};

static CACHED_SUMMONERS_JSON: Lazy<Mutex<Cache<String, Summoners>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl DataDragon {
    /// A cached function to get "summoner.json" from DataDragon
    /// 
    /// # Example
    /// ```
    /// let data_dragon = DataDragon::new(None).await.unwrap();
    /// let champion_json = data_dragon.summoners_json().await;
    /// ```
    pub async fn summoners_json(&self) -> Result<Summoners, DataDragonError> {
        let cache = CACHED_SUMMONERS_JSON.lock().await;
        if let Some(json) = cache.get(&self.language) {
            return Ok(json.clone());
        };

        let url = format!("http://ddragon.leagueoflegends.com/cdn/{}/data/{}/summoner.json", self.version, self.language);
        let request = request::<Summoners, DataDragonError>(
            url.to_owned(),
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await;
        match request {
            Ok(summoners_json) => {
                cache.insert(self.language.clone(), summoners_json.clone()).await;
                cache.sync();
                Ok(summoners_json)
            }
            Err(err) => Err(err),
        }
    }
}
