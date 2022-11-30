use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runes {
    pub id: i64,
    pub name: String,
    pub major_change_patch_version: String,
    pub tooltip: String,
    pub short_desc: String,
    pub long_desc: String,
    pub recommendation_descriptor: String,
    pub icon_path: String,
    pub end_of_game_stat_descs: Vec<String>,
    /// This is always empty, so I just make it optional
    pub recommendation_descriptor_attributes: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunesStyle {
    pub schema_version: i64,
    pub styles: Vec<Style>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub id: i64,
    pub name: String,
    pub tooltip: String,
    pub icon_path: String,
    pub asset_map: Value,
    pub is_advanced: bool,
    pub allowed_sub_styles: Vec<i64>,
    pub sub_style_bonus: Vec<SubStyleBonu>,
    pub slots: Vec<Slot>,
    pub default_page_name: String,
    pub default_sub_style: i64,
    pub default_perks: Vec<i64>,
    pub default_perks_when_splashed: Vec<i64>,
    pub default_stat_mods_per_sub_style: Vec<DefaultStatModsPerSubStyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubStyleBonu {
    pub style_id: i64,
    pub perk_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    #[serde(rename = "type")]
    pub type_field: String,
    pub slot_label: String,
    pub perks: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultStatModsPerSubStyle {
    pub id: String,
    pub perks: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionData {
    pub id: i64,
    pub name: String,
    #[serde(rename = "alias")]
    pub key: String,
    pub square_portrait_path: String,
    pub roles: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionFull {
    pub id: i64,
    pub name: String,
    pub alias: String,
    pub title: String,
    pub short_bio: String,
    pub tactical_info: TacticalInfo,
    pub playstyle_info: PlaystyleInfo,
    pub square_portrait_path: String,
    pub stinger_sfx_path: String,
    pub choose_vo_path: String,
    pub ban_vo_path: String,
    pub roles: Vec<String>,
    pub recommended_item_defaults: Vec<Value>,
    pub skins: Vec<Skin>,
    pub passive: Passive,
    pub spells: Vec<Spell>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TacticalInfo {
    pub style: i64,
    pub difficulty: i64,
    pub damage_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaystyleInfo {
    pub damage: i64,
    pub durability: i64,
    pub crowd_control: i64,
    pub mobility: i64,
    pub utility: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub id: i64,
    pub is_base: bool,
    pub name: String,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub skin_type: String,
    pub rarity: String,
    pub is_legacy: bool,
    pub splash_video_path: Value,
    pub collection_splash_video_path: Value,
    pub features_text: Value,
    pub chroma_path: Option<String>,
    pub emblems: Value,
    pub region_rarity_id: i64,
    pub rarity_gem_path: Value,
    #[serde(default)]
    pub skin_lines: Vec<SkinLine>,
    pub description: Option<String>,
    #[serde(default)]
    pub chromas: Vec<Chroma>,
    pub load_screen_vintage_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinLine {
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chroma {
    pub id: i64,
    pub name: String,
    pub chroma_path: String,
    pub colors: Vec<String>,
    pub descriptions: Vec<Description>,
    pub rarities: Vec<Rarity>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub region: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rarity {
    pub region: String,
    pub rarity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Passive {
    pub name: String,
    pub ability_icon_path: String,
    pub ability_video_path: String,
    pub ability_video_image_path: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    pub spell_key: String,
    pub name: String,
    pub ability_icon_path: String,
    pub ability_video_path: String,
    pub ability_video_image_path: String,
    pub cost: String,
    pub cooldown: String,
    pub description: String,
    pub dynamic_description: String,
    pub range: Vec<f64>,
    pub cost_coefficients: Vec<f64>,
    pub cooldown_coefficients: Vec<f64>,
    pub coefficients: Coefficients,
    pub effect_amounts: EffectAmounts,
    pub ammo: Ammo,
    pub max_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coefficients {
    pub coefficient1: f64,
    pub coefficient2: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectAmounts {
    #[serde(rename = "Effect1Amount")]
    pub effect1amount: Vec<f64>,
    #[serde(rename = "Effect2Amount")]
    pub effect2amount: Vec<f64>,
    #[serde(rename = "Effect3Amount")]
    pub effect3amount: Vec<f64>,
    #[serde(rename = "Effect4Amount")]
    pub effect4amount: Vec<f64>,
    #[serde(rename = "Effect5Amount")]
    pub effect5amount: Vec<f64>,
    #[serde(rename = "Effect6Amount")]
    pub effect6amount: Vec<f64>,
    #[serde(rename = "Effect7Amount")]
    pub effect7amount: Vec<f64>,
    #[serde(rename = "Effect8Amount")]
    pub effect8amount: Vec<f64>,
    #[serde(rename = "Effect9Amount")]
    pub effect9amount: Vec<f64>,
    #[serde(rename = "Effect10Amount")]
    pub effect10amount: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ammo {
    pub ammo_recharge_time: Vec<f64>,
    pub max_ammo: Vec<i64>,
}
