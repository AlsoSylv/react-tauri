use crate::{core::data_dragon::structs::DataDragon, errors::DataDragonError};

use super::structs::ChampionNames;

use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use moka::future::{Cache, ConcurrentCacheExt};

static CACHED_CHAMP_ID: Lazy<Mutex<Cache<String, i64>>> = Lazy::new(|| {
    Mutex::new(Cache::new(25))
});

pub async fn champion_id(name: &str, lang: &str) -> Result<i64, DataDragonError> {
    let cache = CACHED_CHAMP_ID.lock().await;
    let id = cache.get(name);
    if id.is_some() {
        return Ok(id.unwrap());
    }

    let champion_name = format!("{}", name.clone());
    let data_dragon = DataDragon::new(Some(lang)).await;
    match data_dragon {
        Ok(data_dragon) => {
            let request = data_dragon.champion_json().await;
            match request {
                Ok(json) => {
                    let id: i64 = json.data[&champion_name].key.parse().unwrap();
                    cache.insert(name.clone().to_string(), id).await;
                    cache.sync();
                    Ok(id)
                },
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    }
}

pub async fn all_champion_names() -> Result<Vec<ChampionNames>, DataDragonError> {
    let mut champions = Vec::new();
    let data_dragon = DataDragon::new(Some("en_US")).await;
    match data_dragon {
        Ok(data_dragon) => {
            let champ_json = data_dragon.champion_json().await;
            match champ_json {
                Ok(json) => {
                    for (champ_key, champ) in json.data.iter() {
                        let key = &champ.id;
                        champions.push(ChampionNames {
                          label: champ.clone().name,
                          value: champ_key.to_string(),
                          url: format!(
                            "https://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
                            &data_dragon.version,
                            key,
                        ),
                          local_image: format!("/{0}/{0}.png", key),
                        });
                    }
                    Ok(champions)
                }
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    }
}

