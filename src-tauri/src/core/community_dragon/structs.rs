use serde::{Serialize, Deserialize};
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
