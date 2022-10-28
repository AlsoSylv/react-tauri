use cached::proc_macro::cached;
use serde_json::Value;
use phf::phf_map;

use crate::{shared, Active};
use shared::{data_dragon, helpers};

static REGIONS: phf::Map<&'static str, &'static str> = phf_map! {
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
    "world" => "12"
};

static TIERS: phf::Map<&'static str, &'static str> = phf_map! {
    "challenger" => "1",
    "master" => "2",
    "diamond" => "3",
    "platinum" => "4",
    "gold" => "5",
    "silver" => "6",
    "bronze" => "7",
    "overall" => "8",
    "platinum_plus" => "10",
    "diamond_plus" => "11",
    "diamond_2_plus" => "12",
    "grandmaster" => "13",
    "master_plus" => "14",
    "iron" => "15",
};

// Better defaulting logic should be used by wrapping https://stats2.u.gg/lol/1.5/primary_roles/12_20/1.5.0.json
static POSITIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "jungle" => "1",
    "support" => "2",
    "adc" => "3",
    "top" => "4",
    "mid" => "5",
    "none" => "6"
};

static DATA: phf::Map<&'static str, usize> = phf_map! {
    "perks" => 0,
    "summoner_spells" => 1,
    "starting_items" => 2,
    "mythic_and_core" => 3,
    "abilities" => 4,
    "other_items" => 5,
    "shards" => 8,
};

static STATS: phf::Map<&'static str, usize> = phf_map! {
    "wins" => 0,
    "matches" => 1,
    "rank" => 2,
    "total_rank" => 3,
    "bans" => 10,
    "total_matches" => 11,
    "real_matches" => 13,
};

// Investigate wrapping https://stats2.u.gg/lol/1.5/ap-overview/12_20/ranked_solo_5x5/21/1.5.0.json
// UPDATE: This is actually an easy drop in with the current system, but this is not offered to all champiosn.
// Further investigation is needed into finding out which champs this is offered for automatically
#[cached(result = true, size = 1)]
pub async fn overiew_json(name: String) -> Result<String, i64> {
    let stats_version = "1.1";
    let overview_version = "1.5.0";
    let base_overview_url = "https://stats2.u.gg/lol";
    let game_mode = "ranked_solo_5x5";
    let data_dragon_version = data_dragon::data_dragon_version().await;
    match data_dragon_version {
        Ok(version) => {
            let lol_version: Vec<&str> = version.split(".").collect();
            let champion_id = helpers::champion_id(name).await;
            match champion_id {
                Ok(id) => {
                    let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                    let url = format!("{base_overview_url}/{stats_version}/overview/{ugg_lol_version}/{game_mode}/{id}/{overview_version}.json");
                    let request = reqwest::get(url).await;
                    match request {
                        Ok(json) => {
                            let overview = json.text().await;
                            match overview {
                                Ok(valid) => {
                                    Ok(valid)
                                }
                                Err(_) => {
                                    Err(201)
                                }
                            }
                        }
                        Err(err) => {
                            if err.is_body() {
                                Err(202)
                            } else if err.is_request() {
                                Err(201)
                            } else {
                                panic!()
                            }
                        }
                    }
                }
                Err(err) => Err(err)
            }
        }
        Err(err) => Err(err)
    }
}

#[cached(result = true, size = 1)]
async fn ranking_json(name: String) -> Result<String, i64> {
    let stats_version = "1.1";
    let overview_version = "1.5.0";
    let base_overview_url = "https://stats2.u.gg/lol";
    let game_mode = "ranked_solo_5x5";
    let data_dragon_version = data_dragon::data_dragon_version().await;
    match data_dragon_version {
        Ok(version) => {
            let lol_version: Vec<&str> = version.split(".").collect();
            let champion_id = helpers::champion_id(name).await;
            match champion_id {
                Ok(id) => {
                    let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                    let url = format!("{base_overview_url}/{stats_version}/rankings/{ugg_lol_version}/{game_mode}/{id}/{overview_version}.json");
                    let request = reqwest::get(url).await;
                    match request {
                        Ok(json) => {
                            let ranking = json.text().await;
                            match ranking {
                                Ok(valid) => {
                                    Ok(valid)
                                }
                                Err(_) => Err(201)
                            }
                        }
                        Err(err) => {
                            if err.is_connect() {
                                Err(202)
                            } else {
                                panic!()
                            }
                        }
                    }
                }
                Err(err) => Err(err)
            }
        }
        Err(err) => Err(err)
    }
}

//U.GG uses the structure REGION - RANK - ROLE
//For storing things in json, this does the same thing, and uses
//The equivalent match function to change riot API names to U.GG numbers
#[cached(size = 1)]
async fn ranking(name: String, role: String, ranks: String, regions: String) -> Result<Value, i64> {
    let request = ranking_json(name).await;
    match request {
        Ok(ranking) => {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(&ranking);
            match json {
                Ok(json) => {
                    let json_read: &Value = &json[REGIONS[&regions.to_lowercase()]][TIERS[&ranks.to_lowercase()]][POSITIONS[&role.to_lowercase()]];
                    Ok(json_read.to_owned())
                }
                Err(_) => Err(202)
            }
        }
        Err(err) => Err(err)
    }
}

#[cached(size = 1)]
async fn overiew(name: String, role: String, ranks: String, regions: String) -> Result<Value, i64> {
    let request = overiew_json(name).await;
    match request {
        Ok(overview) => {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(&overview);
            match json {
                Ok(json) => {
                    let json_read: &Value = &json[REGIONS[&regions.to_lowercase()]][TIERS[&ranks.to_lowercase()]][POSITIONS[&role.to_lowercase()]][0];
                    Ok(json_read.to_owned())
                }
                Err(_) => Err(202)
            }
        }
        Err(err) => Err(err)
    }
}

//The format is used here to get an exact result from the floating point math
pub async fn winrate(name: String, role: String, ranks: String, regions: String) -> Result<String, i64> {
    let request = ranking(name, role, ranks, regions).await;
    match request {
        Ok(json) => {
            let wins = json[STATS["wins"]].as_f64();
            match wins {
                Some(wins) => {
                    let matches = json[STATS["matches"]].as_f64();
                    match matches {
                        Some(matches) => {
                            let win_rate = wins / matches;
                            Ok(format!("{:.1$}%", win_rate * 100.0, 1))
                        }
                        None => Err(206)
                    }
                }
                None => Err(205)
            }
        }
        Err(err) => Err(err)
    }
}

// These are currently commented out, but should be exposed to the front end eventually
pub async fn banrate(name: String, role: String, ranks: String, regions: String) -> Result<String, i64> {
    let request = ranking(name, role, ranks, regions).await;
    match request {
        Ok(json) => {
            let bans = json[STATS["bans"]].as_f64();
            match bans {
                Some(bans) => {
                    let matches = json[STATS["matches"]].as_f64();
                    match matches {
                        Some(matches) => {
                            let ban_rate = bans / matches;
                            Ok(format!("{:.1$}%", ban_rate * 100.0, 1))
                        }
                        None => Err(206)
                    }
                }
                None => Err(205),
            }
        }
        Err(err) => Err(err)
    }
}

pub async fn pickrate(name: String, role: String, ranks: String, regions: String) -> Result<String, i64> {
    let request = ranking(name, role, ranks, regions).await;
    match request {
        Ok(json) => {
            let picks = json[STATS["matches"]].as_f64();
            match picks {
                Some(picks) => {
                    let matches = json[STATS["total_matches"]].as_f64();
                    match matches {
                        Some(matches) => {
                            let pick_rate = picks / matches;
                            Ok(format!("{:.1$}%", pick_rate * 100.0, 1))
                        }
                        None => Err(206)
                    }
                }
                None => Err(205),
            }
        }
        Err(err) => Err(err)
    }
}

#[cached(result = true, size = 5)]
pub async fn rune_tuple(name: String, role: String, ranks: String, regions: String) -> Result<([Vec<Active>; 2], [Vec<i64>; 2], [i64; 2]), i64> {
    let request = data_dragon::runes_json().await;
    match request {
        Ok(data_dragon_runes_json) => {
            if role == "none" {
                return Err(106);
            } else {

                let request = overiew(name, role, ranks, regions).await;
                match request {
                    Ok(json) => {
                        let json = &json[DATA["perks"]];
                        let rune_ids = &json[4];

                        let rune_tree_id_1: &i64 = &json[2].as_i64().unwrap();
                        let rune_tree_id_2: &i64 = &json[3].as_i64().unwrap();

                        let mut runes_names_1 = ["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()];
                        let mut runes_names_2: Vec<String> = vec!["1".to_owned(), "2".to_owned(), "3".to_owned()];

                        let mut runes_ids_1: [i64; 4] = [1, 2, 3, 4]; //
                        let mut runes_ids_2: Vec<i64> = vec![1, 2, 3];

                        let mut runes_urls_1 = vec!["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()];
                        let mut runes_urls_2: Vec<String> = vec!["1".to_owned(), "2".to_owned(), "4".to_owned()];
                    
                        for tree in data_dragon_runes_json {
                            if &tree.id == rune_tree_id_1 || &tree.id == rune_tree_id_2 {
                                for (slot_position, slots) in tree.slots.iter().enumerate() {
                                    for rune_data in slots.runes.iter() {
                                        for y in 0..6 {
                                            if &tree.id == rune_tree_id_1 {
                                                if rune_ids[y] == rune_data.id {
                                                    runes_names_1[slot_position] = rune_data.clone().name;
                                                    runes_ids_1[slot_position] = rune_data.id;
                                                    runes_urls_1[slot_position] = "http://ddragon.leagueoflegends.com/cdn/img/".to_string() + &rune_data.icon;
                                                }
                                            } else if &tree.id == rune_tree_id_2 && slot_position != 0 {
                                                if rune_ids[y] == rune_data.id {
                                                    runes_names_2[slot_position - 1] = rune_data.clone().name;
                                                    runes_ids_2[slot_position - 1] = rune_data.id;
                                                    runes_urls_2[slot_position - 1] = "http://ddragon.leagueoflegends.com/cdn/img/".to_string() + &rune_data.icon;
                                                }
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
                                runes_urls_2.remove(y);
                            }
                        }
                    
                        for y in 0..3 {
                            if runes_ids_2.len() == 2 {
                                break;
                            } else if runes_ids_2[y] == y as i64 + 1 {
                                runes_ids_2.remove(y);
                            }
                        }

                        let mut rune_names_one: Vec<Active> = Vec::new();
                        let mut rune_names_two: Vec<Active> = Vec::new();

                        for (position, name) in runes_names_1.iter().enumerate() {
                            rune_names_one.push(Active { name: name.to_string(), image: runes_urls_1[position].clone(), active: true })
                        }

                        for (position, name) in runes_names_2.iter().enumerate() {
                            rune_names_two.push(Active { name: name.to_string(), image: runes_urls_2[position].clone(), active: true })
                        }

                        let rune_ids = [runes_ids_1.to_vec(), runes_ids_2];
                        let rune_names = [rune_names_one, rune_names_two];
                        let tree_ids = [rune_tree_id_1.to_owned(), rune_tree_id_2.to_owned()];
                        Ok((rune_names, rune_ids, tree_ids))
            
                    }
                    Err(err) => Err(err)
                }
            }
        }
        Err(err) => Err(err) 
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shards {
    pub row_one: [Shard; 3],
    pub row_two: [Shard; 3],
    pub row_three: [Shard; 3]
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shard {
    pub name: String,
    pub id: i64,
    pub image: String,
    pub active: bool
}

// This needs to be moved to a new structure for returning this data
// And should follow the structure runes do
pub async fn shard_tuple(name: String, role: String, ranks: String, regions: String) -> Result<Shards, i64> {
    let armor = Shard { 
        name: "Armor".to_owned(), 
        id: 5002, 
        image: "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsArmorIcon.png".to_owned(), 
        active: false 
    };

    let magic_resist = Shard { 
        name: "Magic Resist".to_owned(), 
        id: 5003, 
        image: "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsMagicResIcon.png".to_owned(), 
        active: false 
    };

    let health = Shard { 
        name: "Health".to_owned(), 
        id: 5001, 
        image: "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsHealthScalingIcon.png".to_owned(), 
        active: false 
    };

    let adaptive_force = Shard {
        name: "Adaptive Force".to_owned(), 
        id: 5008, 
        image: "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsAdaptiveForceIcon.png".to_owned(), 
        active: false 
    };

    let attack_speed = Shard { 
        name: "Attack Speed".to_owned(), 
        id: 5005, 
        image: "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsAttackSpeedIcon.png".to_owned(), 
        active: false 
    };

    let ability_haste = Shard { 
        name: "Ability Haste".to_owned(), 
        id: 5007, 
        image: "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsCDRScalingIcon.png".to_owned(), 
        active: false 
    };


    let shards: Shards = Shards {
        row_one: [ adaptive_force.clone(), attack_speed, ability_haste ],
        row_two: [ adaptive_force, armor.clone(), magic_resist.clone() ],
        row_three: [ health, armor, magic_resist ]
    };

    let mut mutable_shards = shards.clone();

    let request = overiew(name, role, ranks, regions).await;
    match request {
        Ok(json) => {
            let active_shards = json[DATA["shards"]][2].as_array();
            match active_shards {
                Some(active_shards) => {
                    for (y, shard) in shards.row_one.iter().enumerate() {
                        if shard.id.to_string() == active_shards[0] {
                            mutable_shards.row_one[y] = Shard { name: shard.name.clone(), id: shard.id, image: shard.image.clone(), active: true }
                        }
                    }

                    for (y, shard) in shards.row_two.iter().enumerate() {
                        if shard.id.to_string() == active_shards[1] {
                            mutable_shards.row_two[y] = Shard { name: shard.name.clone(), id: shard.id, image: shard.image.clone(), active: true }
                        }
                    }

                    for (y, shard) in shards.row_three.iter().enumerate() {
                        if shard.id.to_string() == active_shards[2] {
                            mutable_shards.row_three[y] = Shard { name: shard.name.clone(), id: shard.id, image: shard.image.clone(), active: true }
                        }
                    }

                    Ok(mutable_shards)
                },
                None => Err(202)
            }
        }
        Err(err) => Err(err)
    }
}