use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `perks.json` strucutre for Community Dragon
/// 
/// Does not have an order, is not in trees
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runes {
    /// The rune paths ID
    pub id: i64,
    /// The rune paths name
    pub name: String,
    /// The last major patch that changed the rune
    pub major_change_patch_version: String,
    /// This is the Long Description with variables like "@GraceWindow.2@s"
    /// 
    /// Use long_desc instead 
    pub tooltip: String,
    /// Short explanations of runes
    pub short_desc: String,
    /// Descritption with HTML and full explanations of runes
    pub long_desc: String,
    /// This is always blank
    pub recommendation_descriptor: String,
    /// This icon path seems to be wrong, there is no CDN point for runes
    pub icon_path: String,
    /// Variables like "Damage Dealt: @eogvar1@"
    pub end_of_game_stat_descs: Vec<String>,
    /// This is always empty, so I just make it optional
    pub recommendation_descriptor_attributes: Option<Value>,
}

/// `runes-style.json` structure for Community Dragon
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunesStyle {
    pub schema_version: i64,
    pub styles: Vec<Style>,
}

/// Contains all the data on a specific rune path, ie: Domination, Resolve, Inspiration
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    /// Rune tree ID
    pub id: i64,
    /// Name of the rune tree
    pub name: String,
    /// Hover tooltip from client
    pub tooltip: String,
    /// I cannot figure out how to get these icons to works, there is no CDN link
    pub icon_path: String,
    /// This is an asset map for all runes from Community Dragon, I cannot figure
    /// out how to get this to work, there is no CDN link
    pub asset_map: Value,
    /// Not sure what this means, seems to only be true for inspiration
    pub is_advanced: bool,
    /// Other trees that are allowed to be used with the current tree
    pub allowed_sub_styles: Vec<i64>,
    /// Not sure why this would be useful over allowed_sub_styles
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
    /// Unlike Data Dragon, this is the proper champion ID, not the Key
    pub id: i64,
    /// This is the champs proper name, ie: "Bel'Veth"
    pub name: String,
    /// This is the champs proper key, ie: "BelVeth"
    /// 
    /// This is renamed from "alias"
    #[serde(rename = "alias")]
    pub key: String,
    /// This path is not for raw or cdn, I reccomend using "https://cdn.communitydragon.org/latest/champion/{id}/square"
    /// or "https://cdn.communitydragon.org/latest/champion/{key}/square" instead
    pub square_portrait_path: String,
    /// Roles such as "mage", "fighter", "tank", etc
    pub roles: Vec<String>,
}
