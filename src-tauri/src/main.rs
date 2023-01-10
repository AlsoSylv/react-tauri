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
fn roles() -> Vec<&'static str> {
    vec!["Default", "Top", "Jungle", "Mid", "ADC", "Support"]
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
