#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::{HashMap, BTreeMap};

use extensions::ugg::constants;

use constants::ROLES;
// use linked_hash_map::LinkedHashMap;
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
fn roles() -> Vec<String> {
    let mut roles = Vec::new();
    roles.push("Default".to_string());
    for (key, _value) in &ROLES {
        roles.push(key.to_string());
    }
    roles
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

fn get_translatiosn(lang: &str) -> Translations {
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
    pub ranks: BTreeMap<String, String>
}
