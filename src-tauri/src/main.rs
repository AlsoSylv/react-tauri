#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            roles,
            tiers,
            regions,
            logic::all_champion_names,
            logic::get_languages,
            logic::runes_and_abilities,
            logic::champion_info,
            logic::push_runes,
            logic::push_items,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Generates a list and sends it to the front end
#[tauri::command]
fn roles() -> Vec<Role<'static>> {
    vec![
        Role {
            id: "4",
            local_path: "",
            url: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-career-stats/global/default/position_top.png",
        },
        Role {
            id: "1",
            local_path: "",
            url: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-career-stats/global/default/position_jungle.png",
        },
        Role {
            id: "5",
            local_path: "",
            url: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-career-stats/global/default/position_mid.png",
        },
        Role {
            id: "3",
            local_path: "",
            url: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-career-stats/global/default/position_bottom.png",
        },
        Role {
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
}

/// Generates a list and sends it to the front end
#[tauri::command]
fn tiers(lang: &str) -> LinkedHashMap<String, String> {
    get_translatiosn(lang).ranks
}

/// Generates a list and sends it to the front end
#[tauri::command]
fn regions(lang: &str) -> LinkedHashMap<String, String> {
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
    pub regions: LinkedHashMap<String, String>,
    pub ranks: LinkedHashMap<String, String>,
}
