use cached::proc_macro::cached;
use serde::Deserialize;
use serde::Serialize;
use linked_hash_map::LinkedHashMap;

#[cached(result = true)]
pub async fn data_dragon_version() -> Result<String, reqwest::Error> {
    let request: Result<Vec<String>, reqwest::Error> = reqwest::get("https://static.u.gg/assets/lol/riot_patch_update/prod/versions.json").await?.json().await;
    let version = &request.unwrap()[0];
    Ok(version.to_string())
}

#[cached(result = true)]
pub async fn runes_json() -> Result<Root, reqwest::Error> {
    let url = format!("https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/runesReforged.json", data_dragon_version().await.unwrap());
    let request: Result<Root, reqwest::Error> = reqwest::get(&url).await?.json().await;
    Ok(request.unwrap())
}

type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root2 {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<Slot>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub runes: Vec<Rune>
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
pub async fn champion_json() -> ChampJson {
    let data_dragon_version = data_dragon_version().await.unwrap();
    let url = format!("https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json", data_dragon_version);
    let request: Result<ChampJson, reqwest::Error> = reqwest::get(url).await.unwrap().json().await;
    request.unwrap()
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
    Float(f64)
}

impl Default for StatValue {
    fn default() -> Self {
        Self::Integer(0)
    }
}