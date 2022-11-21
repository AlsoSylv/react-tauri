use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use linked_hash_map::LinkedHashMap;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::DataDragonError;

pub struct DataDragon {
    pub version: String,
    pub language: String,
    pub client: reqwest::Client,
}

static CACHED_VERSION: Lazy<Mutex<Cache<String, String>>> = Lazy::new(|| {
    Mutex::new(Cache::new(1))
});

impl DataDragon {
    pub async fn new(language: Option<&str>) -> Result<Self, DataDragonError> {
        let lang = match language {
            Some(lang) => lang,
            None => "en_US",
        };

        let client = reqwest::Client::new();
        let cache = CACHED_VERSION.lock().await;
        if let Some(cache) = cache.get("version") {
            return Ok(
                DataDragon { 
                    version: cache.clone(), 
                    language: lang.to_string(), 
                    client 
                });
        }

        let version = client.get("https://ddragon.leagueoflegends.com/api/versions.json").send().await;
        match version {
            Ok(response) => {
                if let Ok(json) = response.json::<Vec<String>>().await {
                    let version = json[0].clone();
                    cache.insert("version".to_string(), version.clone()).await;
                    Ok(
                        DataDragon { 
                            version, 
                            language: lang.to_string(), 
                            client
                        })
                } else {
                    unreachable!()
                }
            },
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuneTree {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<Slot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub runes: Vec<Rune>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub short_desc: String,
    pub long_desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampJson {
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub version: String,
    pub data: LinkedHashMap<String, ChampData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampData {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub blurb: String,
    pub info: ChampInfo,
    pub image: ChampImage,
    pub tags: Vec<String>,
    pub partype: String,
    pub stats: ChampStats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampInfo {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampImage {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampStats {
    pub hp: StatValue,
    pub hpperlevel: StatValue,
    pub mp: StatValue,
    pub mpperlevel: StatValue,
    pub movespeed: StatValue,
    pub armor: StatValue,
    pub armorperlevel: StatValue,
    pub spellblock: StatValue,
    pub spellblockperlevel: StatValue,
    pub attackrange: StatValue,
    pub hpregen: StatValue,
    pub hpregenperlevel: StatValue,
    pub mpregen: StatValue,
    pub mpregenperlevel: StatValue,
    pub crit: StatValue,
    pub critperlevel: StatValue,
    pub attackdamage: StatValue,
    pub attackdamageperlevel: StatValue,
    pub attackspeedperlevel: StatValue,
    pub attackspeed: StatValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatValue {
    Integer(i64),
    Float(f64),
}

impl Default for StatValue {
    fn default() -> Self {
        Self::Integer(0)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionFull {
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub version: String,
    pub data: Value,
}
