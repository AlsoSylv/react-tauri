use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct CommunityDragon {
    pub language: String,
    pub client: reqwest::Client,
}

impl CommunityDragon {
    pub fn new(client: reqwest::Client, lang: &str) -> Self {
        let binding = lang.to_lowercase();
        let language = match lang {
            "en_US" => "default",
            _ => &binding,
        }
        .to_owned();

        CommunityDragon { language, client }
    }

    pub fn new_with_client(lang: &str) -> Self {
        let client = reqwest::Client::new();
        let binding = lang.to_lowercase();
        let language = match lang {
            "en_US" => "default",
            _ => &binding,
        }
        .to_owned();

        CommunityDragon { language, client }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    // This is always empty, so I just make it optional
    pub recommendation_descriptor_attributes: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunesStyle {
    pub schema_version: i64,
    pub styles: Vec<Style>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubStyleBonu {
    pub style_id: i64,
    pub perk_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    #[serde(rename = "type")]
    pub type_field: String,
    pub slot_label: String,
    pub perks: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultStatModsPerSubStyle {
    pub id: String,
    pub perks: Vec<i64>,
}
