use std::collections::HashMap;

use crate::{core::{data_dragon, helpers}, extensions::ugg::structs};
use helpers::champs::champion_id;

impl structs::UggRequest {
    pub async fn default_role(&self) -> Result<String, i64> {
        let stat_version = "1.5";
        let base_role_url = "https://stats2.u.gg/lol";
        let role_version = "1.5.0";
        let future_data_dragon_version = data_dragon::structs::DataDragon::new(None);
        let future_champion_id = champion_id(&self.name);
        let client = &self.client;
        let (
            data_dragon_version, 
            champion_id,
        ) = futures::join!(
            future_data_dragon_version, 
            future_champion_id,
        );
        match data_dragon_version {
            Ok(data_dragon) => {
                let lol_version: Vec<&str> = data_dragon.version.split(".").collect();
                let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                let url = format!("{base_role_url}/{stat_version}/primary_roles/{ugg_lol_version}/{role_version}.json");
                let request = client.get(url).send().await;
    
                match champion_id {
                    Ok(id) => {
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

    // Investigate wrapping https://stats2.u.gg/lol/1.5/ap-overview/12_20/ranked_solo_5x5/21/1.5.0.json
    // UPDATE: This is actually an easy drop in with the current system, but this is not offered to all champions.
    // Further investigation is needed into finding out which champs this is offered for automatically
    pub async fn overview_json(&self) -> Result<String, i64> {
        let stats_version = "1.5";
        let overview_version = "1.5.0";
        let base_overview_url = "https://stats2.u.gg/lol";
        let game_mode = "ranked_solo_5x5";
    
        let future_data_dragon_version = data_dragon::structs::DataDragon::new(None);
        let future_champion_id = champion_id(&self.name);
        let client = &self.client;
        let (
            data_dragon_version, 
            champion_id
        ) = futures::join!(
            future_data_dragon_version, 
            future_champion_id
        );
    
        match data_dragon_version {
            Ok(data_dragon) => {
                let lol_version: Vec<&str> = data_dragon.version.split(".").collect();
                match champion_id {
                    Ok(id) => {
                        let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                        let url = format!("{base_overview_url}/{stats_version}/overview/{ugg_lol_version}/{game_mode}/{id}/{overview_version}.json");
                        let request = client.get(url).send().await;
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

    pub async fn ranking_json(&self) -> Result<String, i64> {
        let stats_version = "1.5";
        let overview_version = "1.5.0";
        let base_overview_url = "https://stats2.u.gg/lol";
        let game_mode = "ranked_solo_5x5";
    
        let future_data_dragon_version = data_dragon::structs::DataDragon::new(None);
        let future_champion_id = champion_id(&self.name);
        let client = &self.client;
        let (
            data_dragon_version, 
            champion_id
        ) = futures::join!(
            future_data_dragon_version, 
            future_champion_id
        );
    
        match data_dragon_version {
            Ok(data_dragon) => {
                let lol_version: Vec<&str> = data_dragon.version.split(".").collect();
                match champion_id {
                    Ok(id) => {
                        let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                        let url = format!("{base_overview_url}/{stats_version}/rankings/{ugg_lol_version}/{game_mode}/{id}/{overview_version}.json");
                        let request = client.get(url).send().await;
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
}
