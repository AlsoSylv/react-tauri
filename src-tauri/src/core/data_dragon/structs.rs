use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Serialize `runesReforged.json` to a struct
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuneTree {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<Slot>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slot {
    pub runes: Vec<Rune>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub short_desc: String,
    pub long_desc: String,
}

/// Serialize `champions.json` to a struct
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChampInfo {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// Used for more specific info on champions, because it is not guaranteed to be an int or a float
/// this could be replaced with a value, and probably should be in the future.
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

/// Serialize `championFull.json` to a struct
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionFull {
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub version: String,
    pub data: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summoners {
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: String,
    pub data: HashMap<String, SummonerSpell>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub maxrank: i64,
    pub cooldown: Vec<i64>,
    pub cooldown_burn: String,
    pub cost: Vec<i64>,
    pub cost_burn: String,
    pub datavalues: Option<Value>,
    pub effect: Vec<Option<Vec<Value>>>,
    pub effect_burn: Vec<Option<String>>,
    pub vars: Vec<Value>,
    pub key: String,
    pub summoner_level: i64,
    pub modes: Vec<String>,
    pub cost_type: String,
    pub maxammo: String,
    pub range: Vec<i64>,
    pub range_burn: String,
    pub image: Image,
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}
