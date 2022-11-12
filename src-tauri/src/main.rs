#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use cached::proc_macro::cached;
use plugins::{ugg::{structs, constants}, lcu};

use structs::{Shards, Data, ItemsMap, AbilitiesMap};
use constants::{TIERS, REGIONS, ROLES};
use frontend_types::ChampionInfo;

use lcu::{push_runes_to_client};
use shared::helpers::{ChampionNames, create_rune_page};

mod plugins;
mod shared;
mod logic;
pub mod frontend_types;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            rune_names,
            champion_names,
            shard_names,
            champion_info,
            roles,
            tiers,
            regions,
            items,
            push_runes,
            abilities,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn champion_info(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<ChampionInfo, i64> {

    let info = logic::champion_info(name, role, rank, region).await;
    
    match info {
        Ok(values) => Ok(values),
        Err(err) => Err(err),
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneImages {
    pub primary_runes: PrimaryTree,
    pub secondary_runes: SecondaryTree,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryTree {
    pub slot_one: Vec<Active>,
    pub slot_two: Vec<Active>,
    pub slot_three: Vec<Active>,
    pub slot_four: Vec<Active>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryTree {
    pub slot_one: Vec<Active>,
    pub slot_two: Vec<Active>,
    pub slot_three: Vec<Active>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Active {
    pub name: String,
    pub image: String,
    pub local_image: String,
    pub active: bool,
    pub id: i64,
}

#[tauri::command]
#[cached(result = true, size = 5)]
async fn rune_names(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<RuneImages, i64> {
    let data = Data::new(name.clone(), role.clone(), rank, region);
    let rune_match = data.rune_tuple().await;
    match rune_match {
        Ok((rune_names, _, _)) => Ok(rune_names),
        Err(err) => Err(err),
    }
}

#[tauri::command]
async fn champion_names() -> Result<Vec<ChampionNames>, i64> {
    let request = shared::helpers::all_champion_names().await;
    match request {
        Ok(names) => Ok(names),
        Err(err) => Err(err),
    }
}

#[tauri::command]
async fn shard_names(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<Shards, i64> {
    let data = Data::new(name.clone(), role.clone(), rank, region);
    let shards = data.shard_tuple().await;
    match shards {
        Ok(shards) => Ok(shards),
        Err(err) => Err(err),
    }
}

#[tauri::command]
async fn items(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<ItemsMap, i64> {
    let data = Data::new(name.clone(), role.clone(), rank, region);
    let items = data.items().await;
    match items {
        Ok(items) => Ok(items),
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
async fn push_runes(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<i64, i64> {
    let data = Data::new(name.clone(), role.clone(), rank, region);
    let winrate = data.winrate().await;
    let rune_match = data.rune_tuple().await;
    // let (winrate, rune_match) = futures::join!(fut_winrate, fut_rune_match);

    match rune_match {
        Ok((_, tree_ids, rune_ids)) => {
            match winrate {
                Ok(win_rate) => {
                    let page = create_rune_page(format!("{0} {1} {2}", name, role, win_rate), tree_ids[0], tree_ids[1], rune_ids).await;
                    let result = push_runes_to_client(page).await;
                    match result {
                        Ok(ok) => Ok(ok),
                        Err(err) => Err(err)
                    }
                }
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}

#[tauri::command]
async fn abilities(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<AbilitiesMap, i64> {
    let data = Data::new(name.clone(), role.clone(), rank, region);
    let abilties = data.abilities().await;

    match abilties {
        Ok(abilities) => Ok(abilities),
        Err(err) => Err(err)
    }
}
