use super::structs::{self, ChampJson, ChampionFull};

use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use moka::future::{Cache, ConcurrentCacheExt};

static CACHED_CHAMP_JSON: Lazy<Mutex<Cache<String, ChampJson>>> = Lazy::new(|| {
    Mutex::new(Cache::new(3))
});

impl structs::DataDragon {
    pub async fn champion_json(&self) -> Result<ChampJson, i64> {
        let cache = CACHED_CHAMP_JSON.lock().await;
        let json = cache.get(&self.language);
        if json.is_some() {
            return Ok(json.unwrap());
        }
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion.json",
            &self.version,
            &self.language
        );
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let champ_json: Result<ChampJson, reqwest::Error> = response.json().await;
                match champ_json {
                    Ok(champ_json) => {
                        cache.insert(self.language.clone(), champ_json.clone()).await;
                        cache.sync();
                        Ok(champ_json)
                    },
                    Err(_) => Err(103),
                }
            }
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

static CACHED_CHAMP_FULL: Lazy<Mutex<Cache<String, ChampionFull>>> = Lazy::new(|| {
    Mutex::new(Cache::new(3))
});

impl structs::DataDragon {
    pub async fn champ_full(&self, name: String) -> Result<ChampionFull, i64> {
        let cache = CACHED_CHAMP_FULL.lock().await;
        let json = cache.get(&self.language);
        if json.is_some() {
            return Ok(json.unwrap());
        }
        let url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion/{}.json",
            &self.version,
            &self.language,
            &name
        );
        let request = self.client.get(url).send().await;

        match request {
            Ok(response) => {
                let champ_full: Result<ChampionFull, reqwest::Error> = response.json().await;
                match champ_full {
                    Ok(champ_full) =>{
                        cache.insert(self.language.clone(), champ_full.clone()).await;
                        cache.sync();
                        Ok(champ_full)
                    },
                    Err(_) => panic!()
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
