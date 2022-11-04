use std::collections::HashMap;

use cached::proc_macro::cached;
use linked_hash_map::LinkedHashMap;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

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
            let url = format!("https://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/champion.json");
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

pub async fn item_json() -> Result<Items, i64> {
    let data_dragon_version = data_dragon_version().await;
    match data_dragon_version {
        Ok(version) => {
            let url = format!("https://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/champion.json");
            let request = reqwest::get(url).await;
            match request {
                Ok(response) => {
                    let item_json: Result<Items, reqwest::Error> = response.json().await;
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: String,
    pub basic: ItemBasic,
    pub data: ItemData,
    pub groups: Vec<ItemGroup>,
    pub tree: Vec<ItemTree>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemBasic {
    pub name: String,
    pub rune: Rune2,
    pub gold: Gold,
    pub group: String,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    pub consumed: bool,
    pub stacks: i64,
    pub depth: i64,
    pub consume_on_full: bool,
    pub from: Vec<Value>,
    pub into: Vec<Value>,
    pub special_recipe: i64,
    pub in_store: bool,
    pub hide_from_all: bool,
    pub required_champion: String,
    pub required_ally: String,
    pub stats: ItemStats,
    pub tags: Vec<Value>,
    pub maps: ItemMaps,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemStats {
    #[serde(rename = "FlatHPPoolMod")]
    pub flat_hppool_mod: i64,
    #[serde(rename = "rFlatHPModPerLevel")]
    pub r_flat_hpmod_per_level: i64,
    #[serde(rename = "FlatMPPoolMod")]
    pub flat_mppool_mod: i64,
    #[serde(rename = "rFlatMPModPerLevel")]
    pub r_flat_mpmod_per_level: i64,
    #[serde(rename = "PercentHPPoolMod")]
    pub percent_hppool_mod: i64,
    #[serde(rename = "PercentMPPoolMod")]
    pub percent_mppool_mod: i64,
    #[serde(rename = "FlatHPRegenMod")]
    pub flat_hpregen_mod: i64,
    #[serde(rename = "rFlatHPRegenModPerLevel")]
    pub r_flat_hpregen_mod_per_level: i64,
    #[serde(rename = "PercentHPRegenMod")]
    pub percent_hpregen_mod: i64,
    #[serde(rename = "FlatMPRegenMod")]
    pub flat_mpregen_mod: i64,
    #[serde(rename = "rFlatMPRegenModPerLevel")]
    pub r_flat_mpregen_mod_per_level: i64,
    #[serde(rename = "PercentMPRegenMod")]
    pub percent_mpregen_mod: i64,
    #[serde(rename = "FlatArmorMod")]
    pub flat_armor_mod: i64,
    pub r_flat_armor_mod_per_level: i64,
    #[serde(rename = "PercentArmorMod")]
    pub percent_armor_mod: i64,
    pub r_flat_armor_penetration_mod: i64,
    pub r_flat_armor_penetration_mod_per_level: i64,
    pub r_percent_armor_penetration_mod: i64,
    pub r_percent_armor_penetration_mod_per_level: i64,
    #[serde(rename = "FlatPhysicalDamageMod")]
    pub flat_physical_damage_mod: i64,
    pub r_flat_physical_damage_mod_per_level: i64,
    #[serde(rename = "PercentPhysicalDamageMod")]
    pub percent_physical_damage_mod: i64,
    #[serde(rename = "FlatMagicDamageMod")]
    pub flat_magic_damage_mod: i64,
    pub r_flat_magic_damage_mod_per_level: i64,
    #[serde(rename = "PercentMagicDamageMod")]
    pub percent_magic_damage_mod: i64,
    #[serde(rename = "FlatMovementSpeedMod")]
    pub flat_movement_speed_mod: i64,
    pub r_flat_movement_speed_mod_per_level: i64,
    #[serde(rename = "PercentMovementSpeedMod")]
    pub percent_movement_speed_mod: i64,
    pub r_percent_movement_speed_mod_per_level: i64,
    #[serde(rename = "FlatAttackSpeedMod")]
    pub flat_attack_speed_mod: i64,
    #[serde(rename = "PercentAttackSpeedMod")]
    pub percent_attack_speed_mod: i64,
    pub r_percent_attack_speed_mod_per_level: i64,
    pub r_flat_dodge_mod: i64,
    pub r_flat_dodge_mod_per_level: i64,
    #[serde(rename = "PercentDodgeMod")]
    pub percent_dodge_mod: i64,
    #[serde(rename = "FlatCritChanceMod")]
    pub flat_crit_chance_mod: i64,
    pub r_flat_crit_chance_mod_per_level: i64,
    #[serde(rename = "PercentCritChanceMod")]
    pub percent_crit_chance_mod: i64,
    #[serde(rename = "FlatCritDamageMod")]
    pub flat_crit_damage_mod: i64,
    pub r_flat_crit_damage_mod_per_level: i64,
    #[serde(rename = "PercentCritDamageMod")]
    pub percent_crit_damage_mod: i64,
    #[serde(rename = "FlatBlockMod")]
    pub flat_block_mod: i64,
    #[serde(rename = "PercentBlockMod")]
    pub percent_block_mod: i64,
    #[serde(rename = "FlatSpellBlockMod")]
    pub flat_spell_block_mod: i64,
    pub r_flat_spell_block_mod_per_level: i64,
    #[serde(rename = "PercentSpellBlockMod")]
    pub percent_spell_block_mod: i64,
    #[serde(rename = "FlatEXPBonus")]
    pub flat_expbonus: i64,
    #[serde(rename = "PercentEXPBonus")]
    pub percent_expbonus: i64,
    pub r_percent_cooldown_mod: i64,
    pub r_percent_cooldown_mod_per_level: i64,
    pub r_flat_time_dead_mod: i64,
    pub r_flat_time_dead_mod_per_level: i64,
    pub r_percent_time_dead_mod: i64,
    pub r_percent_time_dead_mod_per_level: i64,
    #[serde(rename = "rFlatGoldPer10Mod")]
    pub r_flat_gold_per10mod: i64,
    pub r_flat_magic_penetration_mod: i64,
    pub r_flat_magic_penetration_mod_per_level: i64,
    pub r_percent_magic_penetration_mod: i64,
    pub r_percent_magic_penetration_mod_per_level: i64,
    #[serde(rename = "FlatEnergyRegenMod")]
    pub flat_energy_regen_mod: i64,
    pub r_flat_energy_regen_mod_per_level: i64,
    #[serde(rename = "FlatEnergyPoolMod")]
    pub flat_energy_pool_mod: i64,
    pub r_flat_energy_mod_per_level: i64,
    #[serde(rename = "PercentLifeStealMod")]
    pub percent_life_steal_mod: i64,
    #[serde(rename = "PercentSpellVampMod")]
    pub percent_spell_vamp_mod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune2 {
    pub isrune: bool,
    pub tier: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gold {
    pub base: i64,
    pub total: i64,
    pub sell: i64,
    pub purchasable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemMaps {
    #[serde(rename = "1")]
    pub n1: bool,
    #[serde(rename = "8")]
    pub n8: bool,
    #[serde(rename = "10")]
    pub n10: bool,
    #[serde(rename = "12")]
    pub n12: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemGroup {
    pub id: String,
    #[serde(rename = "MaxGroupOwnable")]
    pub max_group_ownable: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemTree {
    pub header: String,
    pub tags: Vec<String>,
}

type ItemData = HashMap<String, Value>;
