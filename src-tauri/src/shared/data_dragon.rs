use cached::proc_macro::cached;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub mod structs;
mod runes;
mod champs;

#[cached(result = true)]
pub async fn data_dragon_version() -> Result<String, i64> {
    let request =
        reqwest::get("https://static.u.gg/assets/lol/riot_patch_update/prod/versions.json").await;
    match request {
        Ok(response) => {
            let json: Result<Vec<String>, reqwest::Error> = response.json().await;
            match json {
                Ok(json) => Ok(json[0].clone()),
                Err(_) => panic!(),
            }
        }
        Err(_) => Err(104),
    }
}

pub async fn item_json() -> Result<Value, i64> {
    let data_dragon_version = data_dragon_version().await;
    match data_dragon_version {
        Ok(version) => {
            let url = format!("https://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/item.json");
            let request = reqwest::get(url).await;
            match request {
                Ok(response) => {
                    let item_json: Result<Value, reqwest::Error> = response.json().await;
                    match item_json {
                        Ok(item_json) => Ok(item_json),
                        Err(_) => Err(103),
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
        Err(err) => Err(err)
        
    }
}

pub async fn champ_full(name: String) -> Result<ChampionFull, i64> {
    let data_dragon_version = data_dragon_version().await;
    match data_dragon_version {
        Ok(version) => {
            let url = format!("http://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/champion/{name}.json");
            let request = reqwest::get(url).await;
            match request {
                Ok(response) => {
                    let champ_full: Result<ChampionFull, reqwest::Error> = response.json().await;
                    match champ_full {
                        Ok(champ_full) => Ok(champ_full),
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
        },
        Err(err) => Err(err)
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
