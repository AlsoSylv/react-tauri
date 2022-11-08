#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use cached::proc_macro::cached;
use plugins::{ugg::{Shards, Data, TIERS, REGIONS, ROLES, ItemsMap}, lcu::push_runes_to_client};
use shared::helpers::{ChampionNames, create_rune_page};

mod plugins;
mod shared;

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
    let data = Data {
        name: name.clone(), role, rank, region
    };
    let rune_match = Data::rune_tuple(&data).await;
    match rune_match {
        Ok((rune_names, _, _)) => Ok(rune_names),
        Err(err) => Err(err),
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionInfo {
    url: String,
    local_image: String,
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
    let rates = Data {
        name: name.clone(), role, rank, region
    };
    let fut_winrate = Data::winrate(&rates);
    let fut_pickrate = Data::pick_rate(&rates);
    let fut_banrate = Data::ban_rate(&rates);
    let fut_champion_json = shared::data_dragon::champion_json();
    let fut_version = shared::data_dragon::data_dragon_version();
    let (
        winrate,
        pickrate,
        banrate,
        champion_json,
        version
    ) = futures::join!(
        fut_winrate,
        fut_pickrate,
        fut_banrate,
        fut_champion_json,
        fut_version
    );

    match winrate {
        Ok(win_rate) => {
            match pickrate {
                Ok(pick_rate) => {
                    match banrate {
                        Ok(ban_rate) => {
                            match champion_json {
                                Ok(json) => {
                                    let id = &json.data.get(&name).unwrap().id;
                                    match version {
                                        Ok(version) => {
                                            let url = format!("https://ddragon.leagueoflegends.com/cdn/{version}/img/champion/{id}.png");
                                            let local_image = format!("/{0}/{0}.png", id);
                                            Ok(ChampionInfo {
                                                url,
                                                local_image,
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
    let data = Data {
        name: name.clone(), role, rank, region
    };

    let shards = Data::shard_tuple(&data).await;
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
    let data = Data {
        name: name.clone(), role, rank, region
    };
    let items = Data::items(&data).await;
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
    let data = Data {
        name: name.clone(), role: role.clone(), rank, region
    };

    let winrate = Data::winrate(&data).await;
    let rune_match = Data::rune_tuple(&data).await;
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