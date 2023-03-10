#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::{BTreeMap, HashMap};

use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;

mod core;
pub mod errors;
mod extensions;
pub mod frontend_types;
mod logic;
pub mod templates;
#[cfg(test)]
mod tests;

pub static TRANSLATIONS: Lazy<HashMap<String, Translations>> = Lazy::new(|| {
    let json = include_str!("translation.json");
    serde_json::from_str::<HashMap<String, Translations>>(json).unwrap()
});

pub struct AppState {
    client: reqwest::Client,
    hyper_client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl AppState {
    fn new() -> Self {
        let client = reqwest::Client::new();
        let https = HttpsConnector::new();
        let hyper_client =
            hyper::Client::builder().build::<HttpsConnector<HttpConnector>, hyper::Body>(https);
        Self {
            client,
            hyper_client,
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            roles,
            tiers,
            regions,
            logic::all_champion_names,
            logic::get_languages,
            logic::champion_info,
            logic::push_runes,
            logic::push_items,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Generates a list and sends it to the front end
#[tauri::command]
fn roles(lang: &str) -> Vec<Role<'_>> {
    let roles = get_translatiosn(lang).roles;
    vec![
        Role {
            name: roles.top,
            id: "4",
            local_path: "",
            url: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-career-stats/global/default/position_top.png",
        },
        Role {
            name: roles.jungle,
            id: "1",
            local_path: "",
            url: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-career-stats/global/default/position_jungle.png",
        },
        Role {
            name: roles.mid,
            id: "5",
            local_path: "",
            url: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-career-stats/global/default/position_mid.png",
        },
        Role {
            name: roles.adc,
            id: "3",
            local_path: "",
            url: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-career-stats/global/default/position_bottom.png",
        },
        Role {
            name: roles.supp,
            id: "2",
            local_path: "",
            url: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-career-stats/global/default/position_support.png",
        },
    ]
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Role<'a> {
    id: &'a str,
    local_path: &'a str,
    url: &'a str,
    name: String,
}

/// Generates a list and sends it to the front end
#[tauri::command]
fn tiers(lang: &str) -> BTreeMap<String, String> {
    get_translatiosn(lang).ranks
}

/// Generates a list and sends it to the front end
#[tauri::command]
fn regions(lang: &str) -> BTreeMap<String, String> {
    get_translatiosn(lang).regions
}

pub fn get_translatiosn(lang: &str) -> Translations {
    if let Some(translation) = TRANSLATIONS.get(lang) {
        translation.clone()
    } else {
        TRANSLATIONS.get("en_US").unwrap().clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translations {
    pub regions: BTreeMap<String, String>,
    pub ranks: BTreeMap<String, String>,
    pub roles: Roles,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roles {
    pub top: String,
    pub jungle: String,
    pub mid: String,
    pub adc: String,
    pub supp: String,
}
