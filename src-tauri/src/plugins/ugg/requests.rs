use std::collections::HashMap;

use cached::proc_macro::cached;
use serde_json::Value;

use crate::{shared::{data_dragon, helpers}, plugins::ugg::constants::{ROLES, REGIONS, TIERS}};

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
                            let json: Result<HashMap<String, Vec<i64>>, reqwest::Error> = json.json().await;
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

#[cached(size = 1, result = true)]
async fn position(name: String, role: String) -> Result<String, i64> {
    if role == "Default" {
        let role = default_role(name).await;
        match role {
            Ok(role) => Ok(role),
            Err(err) => Err(err),
        }
    } else {
    let role = ROLES[&role];

        Ok(role.to_string())
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
#[cached(size = 1, result=true)]
async fn ranking(
    name: String, 
    role: String, 
    ranks: String, 
    regions: String
) -> Result<Value, i64> {
    let fut_request = ranking_json(name.clone());
    let fut_role = position(name, role);
    let (request, role) = futures::join!(fut_request, fut_role);
    match request {
        Ok(ranking) => {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(&ranking);
            match json {
                Ok(json) => {
                    match role {
                        Ok(role) => {
                            let json_read: &Value = &json[REGIONS[&regions]]
                                [TIERS[&ranks]][&role];

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

#[cached(size = 1, result=true)]
pub async fn overview(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<Value, i64> {
    let fut_request = overview_json(name.clone());
    let fut_role = position(name, role);
    let (request, role) = futures::join!(fut_request, fut_role);
    match request {
        Ok(overview) => {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(&overview);
            match json {
                Ok(json) => {
                    match role {
                        Ok(role) => {
                            let json_read: &Value = &json[REGIONS[&region]]
                                [TIERS[&rank]][&role][0];
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
