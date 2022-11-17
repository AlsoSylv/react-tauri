#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use extensions::{ugg::constants};

use constants::{TIERS, REGIONS, ROLES};
use frontend_types::{ChampionInfo, RunesAndAbilities};

use crate::core::helpers;
use helpers::structs::ChampionNames;

mod extensions;
mod core;
mod logic;
pub mod frontend_types;
pub mod errors;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            champion_names,
            champion_info,
            roles,
            tiers,
            regions,
            push_runes,
            get_languages,
            runes_and_abilities,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn champion_info(
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: String,
) -> Result<ChampionInfo, i64> {
    let info = logic::champion_info(name, role, rank, region, lang).await;
    match info {
        Ok(values) => Ok(values),
        Err(err) => Err(err),
    }
}

#[tauri::command]
async fn runes_and_abilities(
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: String,
) -> Result<RunesAndAbilities, i64> {
    match logic::runes_and_abilities(name, role, rank, region, lang).await {
        Ok(runes) => Ok(runes),
        Err(err) => Err(err),
    }
}

#[tauri::command]
async fn push_runes(
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: String,
) -> Result<i64, i64> {
    let result = logic::push_runes(name, role, rank, region, lang).await;
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

#[tauri::command]
fn roles() -> Vec<String> {
    let mut roles = Vec::new();
    roles.push("Default".to_string());
    for (key, _value) in &ROLES {
        roles.push(key.to_string());
    }
    return roles
}

#[tauri::command]
fn tiers() -> Vec<String> {
    let mut tiers = Vec::new();
    for (key, _value) in &TIERS {
        tiers.push(key.to_string());
    }
    return tiers
}

#[tauri::command]
fn regions() -> Vec<String> {
    let mut regions = Vec::new();
    for (key, _value) in &REGIONS {
        regions.push(key.to_string());
    }
    return regions
}

#[tauri::command]
async fn get_languages() -> Result<Vec<String>, i64> {
    let langs = logic::languages().await;
    match langs {
        Ok(langs) => Ok(langs),
        Err(err) => Err(err)
    }
}

#[tauri::command]
async fn champion_names(lang: String) -> Result<Vec<ChampionNames>, i64> {
    let request = helpers::champs::all_champion_names(&lang).await;
    match request {
        Ok(names) => Ok(names),
        Err(err) => Err(i64::from(err)),
    }
}
