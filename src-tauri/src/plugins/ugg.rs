use std::collections::HashMap;

use cached::proc_macro::cached;
use phf::{phf_map, phf_ordered_map};
use serde_json::Value;

use crate::{shared, Active, RuneImages};
use shared::{data_dragon, helpers};

// These are used in the U.GG JSON to map the value to the human readable name
// This is done for the purpose of code readability, as well as sanity.
pub static REGIONS: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "World" => "12",
    "North America" => "1",
    "EU West" => "2",
    "EU North" => "4",
    "Korea" => "3",
    "Brazil" => "5",
    "LA North" => "6",
    "LA South" => "7",
    "OCE" => "8",
    "Russia" => "9",
    "Turkey" => "10",
    "Japan" => "11",
};

pub static TIERS: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "Challenger" => "1",
    "Grandmaster" => "13",
    "Master" => "2",
    "Diamond" => "3",
    "Platinum" => "4",
    "Gold" => "5",
    "Silver" => "6",
    "Bronze" => "7",
    "Iron" => "15",
    "Overall" => "8",
    "Master Plus" => "14",
    "Diamond Plus" => "11",
    "Diamond 2 Plus" => "12",
    "Platinum Plus" => "10",
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
    println!("{}", role);
    let role = match role.as_str() {
        "Jungle" => "1",
        "Support" => "2",
        "ADC" => "3",
        "Top" => "4",
        "Mid" => "5",
        _ => &role,
    }
    .to_owned();
    if role == "Default" {
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

#[cached(size = 1, result = true)]
async fn default_role(name: String) -> Result<String, i64> {
    let stat_version = "1.5";
    let base_role_url = "https://stats2.u.gg/lol";
    let role_version = "1.5.0";

    let future_data_dragon_version = data_dragon::data_dragon_version();
    let future_champion_id = helpers::champion_id(name);
    let (
        data_dragon_version, 
        champion_id
    ) = futures::join!(
        future_data_dragon_version, 
        future_champion_id
    );

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

// Investigate wrapping https://stats2.u.gg/lol/1.5/ap-overview/12_20/ranked_solo_5x5/21/1.5.0.json
// UPDATE: This is actually an easy drop in with the current system, but this is not offered to all champions.
// Further investigation is needed into finding out which champs this is offered for automatically
#[cached(result = true, size = 1)]
async fn overview_json(name: String) -> Result<String, i64> {
    let stats_version = "1.5";
    let overview_version = "1.5.0";
    let base_overview_url = "https://stats2.u.gg/lol";
    let game_mode = "ranked_solo_5x5";

    let future_data_dragon_version = data_dragon::data_dragon_version();
    let future_champion_id = helpers::champion_id(name);
    let (
        data_dragon_version, 
        champion_id
    ) = futures::join!(
        future_data_dragon_version, 
        future_champion_id
    );

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
    let stats_version = "1.5";
    let overview_version = "1.5.0";
    let base_overview_url = "https://stats2.u.gg/lol";
    let game_mode = "ranked_solo_5x5";

    let future_data_dragon_version = data_dragon::data_dragon_version();
    let future_champion_id = helpers::champion_id(name);
    let (
        data_dragon_version, 
        champion_id
    ) = futures::join!(
        future_data_dragon_version, 
        future_champion_id
    );

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
async fn ranking(
    name: String, 
    role: String, 
    ranks: String, 
    regions: String
) -> Result<Value, i64> {
    let request = ranking_json(name.clone()).await;
    match request {
        Ok(ranking) => {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(&ranking);
            match json {
                Ok(json) => {
                    let role = position(name, role).await;

                    match role {
                        Ok(role) => {
                            let json_read: &Value = &json[REGIONS[&regions]]
                                [TIERS[&ranks]][role];

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
    rank: String,
    region: String,
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
                            let json_read: &Value = &json[REGIONS[&region]]
                                [TIERS[&rank]][role][0];
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

pub struct Data {
    pub name: String,
    pub role: String,
    pub rank: String,
    pub region: String,
}

impl Data {
    //The format is used here to get an exact result from the floating point math
    pub async fn winrate(&self) -> Result<String, i64> {
        let request = ranking(
            self.name.clone(),
            self.role.clone(),
            self.rank.clone(),
            self.region.clone()).await;
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
    
    pub async fn ban_rate(&self) -> Result<String, i64> {
        let request = ranking(
            self.name.clone(),
            self.role.clone(),
            self.rank.clone(),
            self.region.clone()).await;
        match request {
            Ok(json) => {
                let bans = json[STATS["bans"]].as_f64();
                let matches = json[STATS["total_matches"]].as_f64();

                if bans.is_some() && matches.is_some() {
                    let ban_rate = bans.unwrap() / matches.unwrap();
                    Ok(format!("{:.1$}%", ban_rate * 100.0, 1))
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

    pub async fn pick_rate(&self) -> Result<String, i64> {
        let request = ranking (
            self.name.clone(),
            self.role.clone(),
            self.rank.clone(),
            self.region.clone()).await;    
        match request {
            Ok(json) => {
                let picks = json[STATS["matches"]].as_f64();
                let matches = json[STATS["total_matches"]].as_f64();
                if picks.is_some() && matches.is_some() {
                    let pick_rate = picks.unwrap() / matches.unwrap();
                    Ok(format!("{:.1$}%", pick_rate * 100.0, 1))
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

    pub async fn rune_tuple(&self) -> Result<(RuneImages, [i64; 2]), i64>{
        let request = overview(self.name.clone(), self.role.clone(), self.rank.clone(), self.region.clone()).await;
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
                        Ok((all_runes, [*tree_id_one, *tree_id_two]))
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }    
    }

    pub async fn shard_tuple(&self) -> Result<Shards, i64> {
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
    
        let request = overview(self.name.clone(), self.role.clone(), self.rank.clone(), self.region.clone()).await;
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
}
