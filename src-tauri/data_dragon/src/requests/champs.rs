use crate::{
    request,
    types::{ChampJson, ChampionFull},
    DataDragon, DataDragonError,
};

use moka::future::{Cache, ConcurrentCacheExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static CACHED_CHAMP_JSON: Lazy<Mutex<Cache<String, ChampJson>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

static CACHED_CHAMP_FULL: Lazy<Mutex<Cache<(String, String), ChampionFull>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl DataDragon {
    pub async fn champion_json(&self) -> Result<ChampJson, DataDragonError> {
        let cache = CACHED_CHAMP_JSON.lock().await;
        if let Some(json) = cache.get(&self.language) {
            return Ok(json);
        };
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion.json",
            &self.version, &self.language
        );
        let champ_json = request::<ChampJson, DataDragonError>(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        cache
            .insert(self.language.clone(), champ_json.clone())
            .await;
        cache.sync();
        Ok(champ_json)
    }

    /// Method for getting the full json for a specific champion
    /// requires the key field be the same as the one that would
    /// be found for the champ in the champions.json file
    ///
    /// # Example
    /// ```rust
    /// async fn champion_full() {
    ///     use data_dragon::DataDragon;
    ///
    ///     let data_dragon = DataDragon::new(None).await;
    ///     match data_dragon {
    ///         Ok(data_dragon) => {
    ///             let json = data_dragon.champ_full("Xayah").await;
    ///             match json {
    ///                 Ok(json) => {
    ///                     if let Some(id) = json.data["Xayah"]["key"].as_str() {
    ///                         assert!(id == "498");
    ///                     } else {
    ///                         panic!()
    ///                     };
    ///                 }
    ///                 Err(_) => panic!(),
    ///             }
    ///         }
    ///         Err(_) => panic!(),
    ///     }
    /// }
    /// ```
    pub async fn champ_full(&self, key: &str) -> Result<ChampionFull, DataDragonError> {
        let cache = CACHED_CHAMP_FULL.lock().await;
        if let Some(json) = cache.get(&(self.language.clone(), key.to_string())) {
            return Ok(json);
        };
        let url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion/{}.json",
            &self.version, &self.language, &key
        );
        let full_json = request::<ChampionFull, DataDragonError>(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        cache
            .insert((self.language.clone(), key.to_string()), full_json.clone())
            .await;
        cache.sync();
        Ok(full_json)
    }
}
