use cached::proc_macro::cached;
use linked_hash_map::LinkedHashMap;
use serde::Deserialize;
use serde::Serialize;

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

#[cached]
pub async fn runes_json() -> Result<Runes, i64> {
    let data_dragon_version = data_dragon_version().await;
    match data_dragon_version {
        Ok(data_dragon_version) => {
            let url = format!("https://ddragon.leagueoflegends.com/cdn/{data_dragon_version}/data/en_US/runesReforged.json");
            let request = reqwest::get(&url).await;
            match request {
                Ok(response) => {
                    let rune_json: Result<Runes, reqwest::Error> = response.json().await;
                    match rune_json {
                        Ok(rune_json) => Ok(rune_json),
                        Err(_) => Err(104),
                    }
                }
                Err(_) => Err(104),
            }
        }
        Err(err) => Err(err),
    }
}

type Runes = Vec<RuneTree>;

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
pub struct Rune {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
}

#[cached]
pub async fn champion_json() -> Result<ChampJson, i64> {
    let data_dragon_version = data_dragon_version().await;
    match data_dragon_version {
        Ok(version) => {
            let url = format!(
                "https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json",
                version
            );
            let request = reqwest::get(url).await;
            match request {
                Ok(response) => {
                    let champ_json: Result<ChampJson, reqwest::Error> = response.json().await;
                    match champ_json {
                        Ok(champ_json) => Ok(champ_json),
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
        Err(err) => Err(err),
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampJson {
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub version: String,
    pub data: LinkedHashMap<String, Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub blurb: String,
    pub info: Info,
    pub image: Image,
    pub tags: Vec<String>,
    pub partype: String,
    pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
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
