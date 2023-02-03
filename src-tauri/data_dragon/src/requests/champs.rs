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

impl DataDragon<'_> {
    /// Method for getting the champions.json file
    ///
    /// ```rust
    /// async fn champion_json_test() {
    ///     use data_dragon::DataDragon;
    ///
    ///     let data_dragon = DataDragon::new(None).await;
    ///     match data_dragon {
    ///         Ok(data_dragon) => {
    ///             let json = data_dragon.champion_json().await;
    ///             match json {
    ///                 Ok(json) => {
    ///                     assert!(json.data["Xayah"].key == String::from("498"))
    ///                 }
    ///                 Err(_) => panic!(),
    ///             }
    ///         }
    ///         Err(_) => panic!(),
    ///     }
    /// }
    /// ```
    pub async fn champion_json(&self, version: &str) -> Result<ChampJson, DataDragonError> {
        let cache = CACHED_CHAMP_JSON.lock().await;
        if let Some(json) = cache.get(self.lang) {
            return Ok(json);
        };
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion.json",
            version, self.lang
        );
        let champ_json: ChampJson = request(
            &url,
            self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        cache
            .insert(self.lang.to_string(), champ_json.clone())
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
    pub async fn champ_full(
        &self,
        key: &str,
        version: &str,
    ) -> Result<ChampionFull, DataDragonError> {
        let cache = CACHED_CHAMP_FULL.lock().await;
        if let Some(json) = cache.get(&(self.lang.to_string(), key.to_string())) {
            return Ok(json);
        };
        let url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion/{}.json",
            version, self.lang, &key
        );
        let full_json: ChampionFull = request(
            &url,
            self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        cache
            .insert((self.lang.to_string(), key.to_string()), full_json.clone())
            .await;
        cache.sync();
        Ok(full_json)
    }
}
