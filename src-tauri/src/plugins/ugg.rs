use std::collections::HashMap;

use cached::proc_macro::cached;
use phf::phf_map;
use serde_json::Value;

use crate::{shared, Active, PrimaryTree, RuneImages, SecondaryTree};
use shared::{data_dragon, helpers};

// These are used in the U.GG JSON to map the value to the human readable name
// This is done for the purpose of code readability, as well as sanity.
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

async fn position(name: String, role: String) -> Result<String, i64> {
    let role = match role.as_str() {
        "jungle" => "1",
        "support" => "2",
        "adc" => "3",
        "top" => "4",
        "mid" => "5",
        "default" => "6",
        _ => unreachable!(),
    }
    .to_owned();
    if role == "6" {
        let role = default_role(name).await;
        match role {
            Ok(role) => Ok(role),
            Err(err) => Err(err),
        }
    } else {
        Ok(role)
    }
}

//#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
type Roles = HashMap<String, Vec<i64>>;

async fn default_role(name: String) -> Result<String, i64> {
    let stat_version = "1.5";
    let base_role_url = "https://stats2.u.gg/lol";
    let role_version = "1.5.0";
    let future_data_dragon_version = data_dragon::data_dragon_version();
    let future_champion_id = helpers::champion_id(name);
    let (data_dragon_version, champion_id) =
        futures::join!(future_data_dragon_version, future_champion_id);
    match data_dragon_version {
        Ok(version) => {
            let lol_version: Vec<&str> = version.split(".").collect();
            match champion_id {
                Ok(id) => {
                    let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                    let url = format!("{base_role_url}/{stat_version}/primary_roles/{ugg_lol_version}/{role_version}.json");
                    let request = reqwest::get(url).await;
                    match request {
                        Ok(json) => {
                            let json: Result<Roles, reqwest::Error> = json.json().await;
                            match json {
                                Ok(json) => Ok(json[&id.to_string()][0].to_string()),
                                Err(err) => Err(201),
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
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

// Investigate wrapping https://stats2.u.gg/lol/1.5/ap-overview/12_20/ranked_solo_5x5/21/1.5.0.json
// UPDATE: This is actually an easy drop in with the current system, but this is not offered to all champions.
// Further investigation is needed into finding out which champs this is offered for automatically
#[cached(result = true, size = 1)]
async fn overview_json(name: String) -> Result<String, i64> {
    let stats_version = "1.1";
    let overview_version = "1.5.0";
    let base_overview_url = "https://stats2.u.gg/lol";
    let game_mode = "ranked_solo_5x5";
    let future_data_dragon_version = data_dragon::data_dragon_version();
    let future_champion_id = helpers::champion_id(name);
    let (data_dragon_version, champion_id) =
        futures::join!(future_data_dragon_version, future_champion_id);
    match data_dragon_version {
        Ok(version) => {
            let lol_version: Vec<&str> = version.split(".").collect();
            match champion_id {
                Ok(id) => {
                    let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                    let url = format!("{base_overview_url}/{stats_version}/overview/{ugg_lol_version}/{game_mode}/{id}/{overview_version}.json");
                    let request = reqwest::get(url).await;
                    match request {
                        Ok(json) => {
                            let overview = json.text().await;
                            match overview {
                                Ok(valid) => Ok(valid),
                                Err(_) => Err(201),
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
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

#[cached(result = true, size = 1)]
async fn ranking_json(name: String) -> Result<String, i64> {
    let stats_version = "1.1";
    let overview_version = "1.5.0";
    let base_overview_url = "https://stats2.u.gg/lol";
    let game_mode = "ranked_solo_5x5";
    let future_data_dragon_version = data_dragon::data_dragon_version();
    let future_champion_id = helpers::champion_id(name);
    let (data_dragon_version, champion_id) =
        futures::join!(future_data_dragon_version, future_champion_id);
    match data_dragon_version {
        Ok(version) => {
            let lol_version: Vec<&str> = version.split(".").collect();
            match champion_id {
                Ok(id) => {
                    let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                    let url = format!("{base_overview_url}/{stats_version}/rankings/{ugg_lol_version}/{game_mode}/{id}/{overview_version}.json");
                    let request = reqwest::get(url).await;
                    match request {
                        Ok(json) => {
                            let ranking = json.text().await;
                            match ranking {
                                Ok(valid) => Ok(valid),
                                Err(_) => Err(201),
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
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

//U.GG uses the structure REGION - RANK - ROLE
//For storing things in json, this does the same thing, and uses
//The equivalent match function to change riot API names to U.GG numbers
#[cached(size = 1)]
async fn ranking(name: String, role: String, ranks: String, regions: String) -> Result<Value, i64> {
    let request = ranking_json(name.clone()).await;
    match request {
        Ok(ranking) => {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(&ranking);
            match json {
                Ok(json) => {
                    let role = position(name, role).await;

                    match role {
                        Ok(role) => {
                            let json_read: &Value = &json[REGIONS[&regions.to_lowercase()]]
                                [TIERS[&ranks.to_lowercase()]][role];

                            Ok(json_read.to_owned())
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(_) => Err(202),
            }
        }
        Err(err) => Err(err),
    }
}

#[cached(size = 1)]
async fn overview(
    name: String,
    role: String,
    ranks: String,
    regions: String,
) -> Result<Value, i64> {
    let request = overview_json(name.clone()).await;
    match request {
        Ok(overview) => {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(&overview);
            match json {
                Ok(json) => {
                    let role = position(name, role).await;
                    match role {
                        Ok(role) => {
                            let json_read: &Value = &json[REGIONS[&regions.to_lowercase()]]
                                [TIERS[&ranks.to_lowercase()]][role][0];
                            Ok(json_read.to_owned())
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(_) => Err(202),
            }
        }
        Err(err) => Err(err),
    }
}

pub struct Rates {
    pub name: String,
    pub role: String,
    pub rank: String,
    pub region: String,
}

impl Rates {
    pub async fn winrate(&self) -> Result<String, i64> {
        let request = ranking(self.name.clone(), self.role.clone(), self.rank.clone(), self.region.clone()).await;
        match request {
            Ok(json) => {
                let wins = json[STATS["wins"]].as_f64();
                let matches = json[STATS["matches"]].as_f64();
                if wins.is_some() && matches.is_some() {
                    let win_rate = wins.unwrap() / matches.unwrap();
                    Ok(format!("{:.1$}%", win_rate * 100.0, 1))
                } else {
                    if matches.is_none() {
                        Err(206)
                    } else {
                        Err(205)
                    }
                }
            }
            Err(err) => Err(err)
        }
    }
}
//The format is used here to get an exact result from the floating point math
pub async fn win_rate(
    name: String,
    role: String,
    ranks: String,
    regions: String,
) -> Result<String, i64> {
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
                        None => Err(206),
                    }
                }
                None => Err(205),
            }
        }
        Err(err) => Err(err),
    }
}

// These are currently commented out, but should be exposed to the front end eventually
pub async fn ban_rate(
    name: String,
    role: String,
    ranks: String,
    regions: String,
) -> Result<String, i64> {
    let request = ranking(name, role, ranks, regions).await;
    match request {
        Ok(json) => {
            let bans = json[STATS["bans"]].as_f64();
            match bans {
                Some(bans) => {
                    let matches = json[STATS["total_matches"]].as_f64();
                    match matches {
                        Some(matches) => {
                            let ban_rate = bans / matches;
                            Ok(format!("{:.1$}%", ban_rate * 100.0, 1))
                        }
                        None => Err(206),
                    }
                }
                None => Err(205),
            }
        }
        Err(err) => Err(err),
    }
}

pub async fn pick_rate(
    name: String,
    role: String,
    ranks: String,
    regions: String,
) -> Result<String, i64> {
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
                        None => Err(206),
                    }
                }
                None => Err(205),
            }
        }
        Err(err) => Err(err),
    }
}

#[cached(result = true, size = 5)]
pub async fn rune_tuple(
    name: String,
    role: String,
    ranks: String,
    regions: String,
) -> Result<(RuneImages, [i64; 2]), i64> {
    if role == "none" {
        return Err(106);
    } else {
        let request = overview(name, role, ranks, regions).await;
        match request {
            Ok(json) => {
                let json = &json[DATA["perks"]];
                let rune_ids = &json[4];

                let tree_id_one: &i64 = &json[2].as_i64().unwrap();
                let tree_id_two: &i64 = &json[3].as_i64().unwrap();

                let all_runes = helpers::all_rune_images(*tree_id_one, *tree_id_two).await;

                match all_runes {
                    Ok(immutable_all_runes) => {
                        let mut all_runes = immutable_all_runes.clone();
                        let mut slots: [&mut Vec<Active>; 7] = [
                            &mut all_runes.primary_runes.slot_one,
                            &mut all_runes.primary_runes.slot_two,
                            &mut all_runes.primary_runes.slot_three,
                            &mut all_runes.primary_runes.slot_four,
                            &mut all_runes.secondary_runes.slot_one,
                            &mut all_runes.secondary_runes.slot_two,
                            &mut all_runes.secondary_runes.slot_three,
                        ];
                        for n in 0..6 {
                            slots.iter_mut().for_each(|current_slot| {
                                current_slot.clone().iter().enumerate().for_each(|i| {
                                    let pos = i.0;
                                    let rune = i.1;
                                    if rune_ids[n] == rune.id {
                                        current_slot[pos] = Active {
                                            name: rune.name.clone(),
                                            image: rune.image.clone(),
                                            active: true,
                                            id: rune.id,
                                        }
                                    }
                                });
                            });
                        }
                        let rune_images = RuneImages {
                            primary_runes: PrimaryTree {
                                slot_one: slots[0].to_vec(),
                                slot_two: slots[1].to_vec(),
                                slot_three: slots[2].to_vec(),
                                slot_four: slots[3].to_vec(),
                            },
                            secondary_runes: SecondaryTree {
                                slot_one: slots[4].to_vec(),
                                slot_two: slots[5].to_vec(),
                                slot_three: slots[6].to_vec(),
                            },
                        };

                        Ok((rune_images, [*tree_id_one, *tree_id_two]))
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shards {
    pub row_one: [Shard; 3],
    pub row_two: [Shard; 3],
    pub row_three: [Shard; 3],
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shard {
    pub name: String,
    pub id: i64,
    pub image: String,
    pub active: bool,
}

// This needs to be moved to a new structure for returning this data
// And should follow the structure runes do
pub async fn shard_tuple(
    name: String,
    role: String,
    ranks: String,
    regions: String,
) -> Result<Shards, i64> {
    let armor = Shard {
        name: "Armor".to_owned(),
        id: 5002,
        image:
            "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsArmorIcon.png"
                .to_owned(),
        active: false,
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
        row_one: [adaptive_force.clone(), attack_speed, ability_haste],
        row_two: [adaptive_force, armor.clone(), magic_resist.clone()],
        row_three: [health, armor, magic_resist],
    };

    let mut mutable_shards = shards.clone();

    let request = overview(name, role, ranks, regions).await;
    match request {
        Ok(json) => {
            let active_shards = json[DATA["shards"]][2].as_array();
            match active_shards {
                Some(active_shards) => {
                    for (y, shard) in shards.row_one.iter().enumerate() {
                        if shard.id.to_string() == active_shards[0] {
                            mutable_shards.row_one[y] = Shard {
                                name: shard.name.clone(),
                                id: shard.id,
                                image: shard.image.clone(),
                                active: true,
                            }
                        }
                    }

                    for (y, shard) in shards.row_two.iter().enumerate() {
                        if shard.id.to_string() == active_shards[1] {
                            mutable_shards.row_two[y] = Shard {
                                name: shard.name.clone(),
                                id: shard.id,
                                image: shard.image.clone(),
                                active: true,
                            }
                        }
                    }

                    for (y, shard) in shards.row_three.iter().enumerate() {
                        if shard.id.to_string() == active_shards[2] {
                            mutable_shards.row_three[y] = Shard {
                                name: shard.name.clone(),
                                id: shard.id,
                                image: shard.image.clone(),
                                active: true,
                            }
                        }
                    }

                    Ok(mutable_shards)
                }
                None => Err(202),
            }
        }
        Err(err) => Err(err),
    }
}
