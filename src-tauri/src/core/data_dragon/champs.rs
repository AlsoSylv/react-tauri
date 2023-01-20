use super::structs::{ChampJson, ChampionFull};

use super::DataDragon;
use crate::templates::request;

use crate::errors::DataDragonError;
use moka::future::{Cache, ConcurrentCacheExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

/// A cache for the `champions.json` file, needs to be changed on lang change
static CACHED_CHAMP_JSON: Lazy<Mutex<Cache<String, ChampJson>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

/// A cache for the champion specific JSON files
/// this needs to be changed on champion name, and on lang chang
static CACHED_CHAMP_FULL: Lazy<Mutex<Cache<(String, String), ChampionFull>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl DataDragon {
    /// A cached function to get the DataDragon champion.json and return it
    /// serialized as a struct for the rest of the code
    ///
    /// # Example
    /// ```
    /// let data_dragon = DataDragon::new(None).await.unwrap();
    /// let champion_json = data_dragon.champion_json().await;
    /// ```
    pub async fn champion_json(&self) -> Result<ChampJson, DataDragonError> {
        let cache = CACHED_CHAMP_JSON.lock().await;
        if let Some(json) = cache.get(&self.language) {
            return Ok(json);
        };

        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion.json",
            &self.version, &self.language
        );
        let request = request::<ChampJson, DataDragonError>(
            url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await;

        match request {
            Ok(champ_json) => {
                cache
                    .insert(self.language.clone(), champ_json.clone())
                    .await;
                cache.sync();
                Ok(champ_json)
            }

            Err(err) => Err(err),
        }
    }

    /// A cached function to get the json files for specific
    /// champions from DataDragon, this requires an extra
    /// argument for the champions Key from champions.json
    ///
    /// # Example
    /// ```
    /// let data_dragon = DataDragon::new(None).await.unwrap();
    /// let champion_json = data_dragon.champ_full("Xayah".to_owned()).await;
    /// ```
    pub async fn champ_full(&self, key: String) -> Result<ChampionFull, DataDragonError> {
        let cache = CACHED_CHAMP_FULL.lock().await;
        if let Some(json) = cache.get(&(self.language.clone(), key.clone())) {
            return Ok(json);
        };

        let url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion/{}.json",
            &self.version, &self.language, &key
        );
        let request = request::<ChampionFull, DataDragonError>(
            url.to_owned(),
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await;

        match request {
            Ok(champ_full) => {
                cache
                    .insert((self.language.clone(), key.clone()), champ_full.clone())
                    .await;
                cache.sync();
                Ok(champ_full)
            }
            Err(err) => Err(err),
        }
    }
}
