#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use plugins::ugg::Shards;
use shared::helpers::ChampionNames;

mod plugins;
mod shared;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            rune_names,
            champion_names,
            shard_names,
            champion_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
pub struct Active {
    pub name: String,
    pub image: String,
    pub active: bool,
    pub id: i64,
}

#[tauri::command]
async fn rune_names(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<RuneImages, i64> {
    // TODO: This can be none if you get data specific enough, I need to handle that
    let rune_match = plugins::ugg::rune_tuple(name, role, rank, region).await;
    match rune_match {
        Ok((rune_names, _tree_ids)) => Ok(rune_names),
        Err(err) => Err(err),
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionInfo {
    url: String,
    win_rate: String,
    pick_rate: String,
    ban_rate: String,
}

#[tauri::command]
async fn champion_info(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<ChampionInfo, i64> {
    let rates = plugins::ugg::Rates {name: name.clone(), role: role.clone(), rank: rank.clone(), region: region.clone()};
    let request = plugins::ugg::Rates::winrate(&rates).await;
    match request {
        Ok(win_rate) => {
            let request =
                plugins::ugg::pick_rate(name.clone(), role.clone(), rank.clone(), region.clone())
                    .await;
            match request {
                Ok(pick_rate) => {
                    let request = plugins::ugg::ban_rate(
                        name.clone(),
                        role.clone(),
                        rank.clone(),
                        region.clone(),
                    )
                    .await;
                    match request {
                        Ok(ban_rate) => {
                            let request = shared::data_dragon::champion_json().await;
                            match request {
                                Ok(json) => {
                                    let id = &json.data.get(&name).unwrap().id;
                                    let request = shared::data_dragon::data_dragon_version().await;
                                    match request {
                                        Ok(version) => {
                                            let url = format!("https://ddragon.leagueoflegends.com/cdn/{version}/img/champion/{id}.png");
                                            Ok(ChampionInfo {
                                                url,
                                                win_rate,
                                                pick_rate,
                                                ban_rate,
                                            })
                                        }
                                        Err(err) => Err(err),
                                    }
                                }
                                Err(err) => Err(err),
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            }
        }
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
    let shards = plugins::ugg::shard_tuple(name, role, rank, region).await;
    match shards {
        Ok(shards) => Ok(shards),
        Err(err) => Err(err),
    }
}
