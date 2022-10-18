use cached::proc_macro::cached;
use reqwest::Error;
use serde_json::Value;

use crate::shared;
use shared::{data_dragon, helpers};

//Move this to a constant HashMap, or an auto generated HashMap
async fn match_region(region: &str) -> &str {
    //Vertify these are the correct names in the Riot API enpoints
    match region {
        "na1" => "1",
        "euw1" => "2",
        "kr" => "3",
        "eun1" => "4",
        "br1" => "5",
        "la1" => "6",
        "la2" => "7",
        "oc1" => "8",
        "ru" => "9",
        "tr1" => "10",
        "jp1" => "11",
        "world" => "12",
        _ => "12"
    }
}

async fn match_tiers(ranks: &str) -> &str {
    match ranks {
        "challenger" => "1",
        "master" => "2",
        "diamond" => "3",
        "platinum" => "4",
        "gold" => "5",
        "silver" => "6",
        "bronze" => "7",
        "overall" => "8",
        "platinum_plus" => "10",
        "diamnond_plus" => "11",
        "diamond_2_plus" => "12",
        "grandmaster" => "13",
        "master_plus" => "14",
        "iron" => "15",
        _ => "10"
    }
}

async fn match_positions(role: &str) -> &str {
    match role {
        "jungle" => "1",
        "support" => "2",
        "adc" => "3",
        "top" => "4",
        "mid" => "5",
        "none" => "6",
        _ => "6"
    }
}

async fn match_data(data: &str) -> usize {
    match data {
        "perks" => 0,
        "summoner_spells" => 1,
        "starting_items" => 2,
        "mythic_and_core" => 3,
        "abilities" => 4,
        "other_items" => 5,
        "shards" => 8,
        _ => unreachable!()
    }
}

async fn match_stats(stats: &str) -> usize {
    match stats {
        "wins" => 0,
        "matches" => 1,
        "rank" => 2,
        "total_rank" => 3,
        "bans" => 10,
        "total_matches" => 11,
        "real_matches" => 13,
        _ => unreachable!()
    }
}

#[cached(result = true, size = 1)]
pub async fn stats(name: String) -> Result<String, reqwest::Error> {
    let stats_version = "1.1";
    let overview_version = "1.5.0";
    let base_overview_url = "https://stats2.u.gg/lol";
    let game_mode = "ranked_solo_5x5";
    let data_dragon_version = data_dragon::data_dragon_version().await.unwrap();
    let lol_version: Vec<&str> = data_dragon_version.split(".").collect();
    let champion_id = helpers::champion_id(name).await.unwrap();
    let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
    let url = format!("{base_overview_url}/{stats_version}/overview/{ugg_lol_version}/{game_mode}/{champion_id}/{overview_version}.json");
    let request = reqwest::get(url).await?.text().await;

    Ok(request.unwrap())
}

#[cached(result = true, size = 1)]
async fn ranking(name: String) -> Result<String, reqwest::Error> {
    let stats_version = "1.1";
    let overview_version = "1.5.0";
    let base_overview_url = "https://stats2.u.gg/lol";
    let game_mode = "ranked_solo_5x5";
    let data_dragon_version = data_dragon::data_dragon_version().await.unwrap();
    let lol_version: Vec<&str> = data_dragon_version.split(".").collect();
    let champion_id = helpers::champion_id(name).await.unwrap();
    let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
    let url = format!("{base_overview_url}/{stats_version}/rankings/{ugg_lol_version}/{game_mode}/{champion_id}/{overview_version}.json");
    let request = reqwest::get(url).await?.text().await;
    Ok(request.unwrap())
}

//U.GG uses the structure REGION - RANK - ROLE
//For storing things in json, this does the same thing, and uses
//The equivalent match function to change riot API names to U.GG numbers
#[cached(size = 1)]
async fn json_read_cache(name: String, role: String, ranks: String, regions: String) -> Value {
    let ranking = ranking(name.to_string()).await.unwrap();
    let json: Value = serde_json::from_str(&ranking).unwrap();
    let json_read = &json[match_region(&regions.to_lowercase()).await][match_tiers(&ranks.to_lowercase()).await][match_positions(&role.to_lowercase()).await];
    return json_read.to_owned()
}

#[cached(size = 1)]
async fn overiew(name: String, role: String, ranks: String, regions: String) -> Value {
    let json: Value = serde_json::from_str(&stats(name.to_string()).await.unwrap()).unwrap();
    let ugg_stats = &json[match_region(&regions.to_lowercase()).await][match_tiers(&ranks.to_lowercase()).await][match_positions(&role.to_lowercase()).await][0];
    return ugg_stats.to_owned()
}

//The format is used here to get an exact result from the floating point math
pub async fn winrate(name: String, role: String, ranks: String, regions: String) -> String {
    let win_rate: f64 = &json_read_cache(name.clone(), role.clone(), ranks.clone(), regions.clone()).await[match_stats("wins").await].as_f64().unwrap() / &json_read_cache(name, role, ranks, regions).await[match_stats("matches").await].as_f64().unwrap();
    return format!("{:.1$}%", win_rate * 100.0, 1)
}

pub async fn banrate(name: String, role: String, ranks: String, regions: String) -> String {
    let ban_rate: f64 = &json_read_cache(name.clone(), role.clone(), ranks.clone(), regions.clone()).await[match_stats("bans").await].as_f64().unwrap() / &json_read_cache(name, role, ranks, regions).await[match_stats("matches").await].as_f64().unwrap();
    return format!("{:.1$}%", ban_rate * 100.0, 1)
}

pub async fn pickrate(name: String, role: String, ranks: String, regions: String) -> String {
    let pick_rate: f64 = &json_read_cache(name.clone(), role.clone(), ranks.clone(), regions.clone()).await[match_stats("matches").await].as_f64().unwrap() / &json_read_cache(name, role, ranks, regions).await[match_stats("total_matches").await].as_f64().unwrap();
    return format!("{:.1$}%", pick_rate * 100.0, 1)
}

#[cached(result = true, size = 5)]
pub async fn two_dimensional_rune_array(name: String, role: String, ranks: String, regions: String) -> Result<([Vec<String>; 2], [Vec<i64>; 2], [Value; 2]), Error> {
    let data_dragon_runes_json = &data_dragon::runes_json().await.unwrap();
    let ugg_runes_json_read = &overiew(name, role, ranks, regions).await[match_data("perks").await];
    let rune_ids = &ugg_runes_json_read[4];
    let rune_tree_id_1 = &ugg_runes_json_read[2];
    let rune_tree_id_2 = &ugg_runes_json_read[3];
    let mut runes_names_1 = ["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()];
    let mut runes_names_2: Vec<String> = vec!["1".to_owned(), "2".to_owned(), "3".to_owned()];
    let mut runes_ids_1: [i64; 4] = [1, 2, 3, 4];
    let mut runes_ids_2: Vec<i64> = vec![1, 2, 3];

    for tree in data_dragon_runes_json {
        if &tree.id == rune_tree_id_1 {
            for (slot_position, slots) in tree.slots.iter().enumerate() {
                for rune_data in slots.runes.iter() {
                    for y in 0..6 {
                        if rune_ids[y] == rune_data.id {
                            runes_names_1[slot_position] = rune_data.clone().name;
                            runes_ids_1[slot_position] = rune_data.id;
                        }
                    }
                }
            }
        } else if &tree.id == rune_tree_id_2 {
            for (slot_position, slots) in tree.slots.iter().enumerate() {
                for rune_data in slots.runes.iter() {
                    for y in 0..6 {
                        if rune_ids[y] == rune_data.id {
                            runes_names_2[slot_position - 1] = rune_data.clone().name;
                            runes_ids_2[slot_position - 1] = rune_data.id;
                        }
                    }
                }
            }
        } 
    }
    for y in 0..3 {
        if runes_names_2.len() == 2 {
            break;
        } else if runes_names_2[y] == (y + 1).to_string() {
            runes_names_2.remove(y);
        }
    }

    for y in 0..3 {
        if runes_ids_2.len() == 2 {
            break;
        } else if runes_ids_2[y] == y as i64 + 1 {
            runes_ids_2.remove(y);
        }
    }
    let rune_ids = [runes_ids_1.to_vec(), runes_ids_2];
    let rune_names = [runes_names_1.to_vec(), runes_names_2];
    let tree_ids = [rune_tree_id_1.to_owned(), rune_tree_id_2.to_owned()];
    Ok((rune_names, rune_ids, tree_ids))
}

pub async fn shard_names(name: String, role: String, ranks: String, regions: String) -> Result<[String; 3], Error> {
    let ugg_shards_json_read = &overiew(name, role, ranks, regions).await[match_data("shards").await][2];
    let mut stat_shard_names: [String; 3] = ["1".to_owned(), "2".to_owned(), "3".to_owned()];
    for (position, name) in ugg_shards_json_read.as_array().unwrap().iter().enumerate() {
        //This really needs some sort of localization system
        match name.as_str().unwrap() {
            "5001" => stat_shard_names[position] = "Health".to_owned(),
            "5008" => stat_shard_names[position] = "Adaptive Force".to_owned(),
            "5007" => stat_shard_names[position] = "Ability Haste".to_owned(),
            "5002" => stat_shard_names[position] = "Armor".to_owned(),
            "5005" => stat_shard_names[position] = "Attack Speed".to_owned(),
            "5003" => stat_shard_names[position] = "Magic Resist".to_owned(),
            _ => unreachable!()
        }
    }
    Ok(stat_shard_names)
}

pub async fn shard_ids(name: String, role: String, ranks: String, regions: String) -> Result<Vec<i64>, Error> {
    let ugg_shards_json: Value = serde_json::from_str(&stats(name).await.unwrap()).unwrap();
    let ugg_shards_json_read = &ugg_shards_json[match_region(&regions.to_lowercase()).await][match_tiers(&ranks.to_lowercase()).await][match_positions(&role.to_lowercase()).await][0][match_data("shards").await][2];
    let mut stat_shard_ids: Vec<i64> = vec![];
    for y in ugg_shards_json_read.as_array().unwrap().iter() {
        stat_shard_ids.push(y.as_str().unwrap().parse::<i64>().unwrap());
    }
    Ok(stat_shard_ids)
}

pub async fn selected_perks_ids_ugg(name: String, role: String, ranks: String, regions: String) -> Result<[i64; 9], Error> {
    let (_names, ids, _treenames) = two_dimensional_rune_array(name.clone(), role.clone(), ranks.clone(), regions.clone()).await.unwrap();
    let shard_id_array = shard_ids(name, role, ranks, regions).await.unwrap();
    let selected_perk_ids = [ids[0][0], ids[0][1], ids[0][2], ids[0][3], ids[1][0], ids[1][1], shard_id_array[0], shard_id_array[1], shard_id_array[2]];
    Ok(selected_perk_ids)
}