use super::structs::{self, ChampJson, ChampionFull};

use crate::errors::DataDragonError;
use moka::future::{Cache, ConcurrentCacheExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static CACHED_CHAMP_JSON: Lazy<Mutex<Cache<String, ChampJson>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl structs::DataDragon {
    pub async fn champion_json(&self) -> Result<ChampJson, DataDragonError> {
        let cache = CACHED_CHAMP_JSON.lock().await;
        let json = cache.get(&self.language);
        if json.is_some() {
            return Ok(json.unwrap());
        }
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion.json",
            &self.version, &self.language
        );
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let champ_json: Result<ChampJson, reqwest::Error> = response.json().await;
                match champ_json {
                    Ok(champ_json) => {
                        cache
                            .insert(self.language.clone(), champ_json.clone())
                            .await;
                        cache.sync();
                        Ok(champ_json)
                    }
                    Err(_) => Err(DataDragonError::ChampMissingError),
                }
            }
            Err(err) => {
                if err.is_body() {
                    Err(DataDragonError::DataDragonMissing)
                } else {
                    Err(DataDragonError::ChampMissingError)
                }
            }
        }
    }
}

static CACHED_CHAMP_FULL: Lazy<Mutex<Cache<(String, String), ChampionFull>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl structs::DataDragon {
    pub async fn champ_full(&self, name: String) -> Result<ChampionFull, DataDragonError> {
        let cache = CACHED_CHAMP_FULL.lock().await;
        let json = cache.get(&(self.language.clone(), name.clone()));
        if json.is_some() {
            return Ok(json.unwrap());
        }
        let url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion/{}.json",
            &self.version, &self.language, &name
        );
        let request = self.client.get(url).send().await;

        match request {
            Ok(response) => {
                let champ_full: Result<ChampionFull, reqwest::Error> = response.json().await;
                match champ_full {
                    Ok(champ_full) => {
                        cache
                            .insert((self.language.clone(), name.clone()), champ_full.clone())
                            .await;
                        cache.sync();
                        Ok(champ_full)
                    }
                    Err(_) => Err(DataDragonError::ChampMissingError),
                }
            }
            Err(err) => {
                if err.is_body() {
                    Err(DataDragonError::DataDragonMissing)
                } else {
                    Err(DataDragonError::ChampMissingError)
                }
            }
        }
    }
}
