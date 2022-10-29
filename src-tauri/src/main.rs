#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use plugins::ugg::Shards;


mod plugins;
mod shared;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler!
        [
        rune_names, 
        champion_names, 
        win_rate, 
        shard_names,
        pick_rate,
        ban_rate
        ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneImages {
    pub primary_runes: PrimaryTree,
    pub secondary_runes: SecondaryTree
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryTree {
    pub slot_one: Vec<Active>,
    pub slot_two: Vec<Active>,
    pub slot_three: Vec<Active>,
    pub slot_four: Vec<Active>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryTree {
    pub slot_one: Vec<Active>,
    pub slot_two: Vec<Active>,
    pub slot_three: Vec<Active>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Active  {
    pub name: String,
    pub image: String,
    pub active: bool,
    pub id: i64
}

#[tauri::command]
async fn rune_names(name: String, role: String, rank: String, region: String) -> Result<RuneImages, i64> {
    // TOOD: This can be none if you get data specific enough, I need to handle that 
    let rune_match = plugins::ugg::rune_tuple(name, role, rank, region).await;
    match rune_match {
        Ok((rune_names, _tree_ids)) => Ok(rune_names),
        Err(err) => Err(err)
    }
}

#[tauri::command]
async fn win_rate(name: String, role: String, rank: String, region: String) -> Result<String, i64> {
    let request = plugins::ugg::winrate(name, role, rank, region).await;
    match request {
    Ok(winrate) => Ok(winrate),
    Err(err) => Err(err)
    }
}

#[tauri::command]
async fn pick_rate(name: String, role: String, rank: String, region: String) -> Result<String, i64> {
    let request = plugins::ugg::pickrate(name, role, rank, region).await;
    match request {
    Ok(winrate) => Ok(winrate),
    Err(err) => Err(err)
    }
}

#[tauri::command]
async fn ban_rate(name: String, role: String, rank: String, region: String) -> Result<String, i64> {
    let request = plugins::ugg::banrate(name, role, rank, region).await;
    match request {
    Ok(winrate) => Ok(winrate),
    Err(err) => Err(err)
    }
}

#[tauri::command]
async fn champion_names() -> Result<Vec<String>, i64> {
    let request = shared::helpers::all_champion_names().await;
    match request {
        Ok(names) => {
            Ok(names)
        }
        Err(err) => Err(err)
    }
}

#[tauri::command]
async fn shard_names(name: String, role: String, rank: String, region: String) -> Result<Shards, i64> {
    let shards = plugins::ugg::shard_tuple(name, role, rank, region).await;
    match shards {
        Ok(shards) => {
            println!("{:#?}", shards);
            Ok(shards)
        }
        Err(err) => Err(err)
    }
}
