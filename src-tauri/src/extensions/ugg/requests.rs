use std::collections::HashMap;
use moka::future::{Cache, ConcurrentCacheExt};
use tokio::sync::Mutex;

use crate::{core::data_dragon, extensions::ugg::structs, errors::{ErrorMap, UGGDataError}};
use ErrorMap::{DataDragonErrors, UGGError};
// use helpers::champs::champion_id;
use once_cell::sync::Lazy;

static CACHED_DEFAULT_ROLE: Lazy<Mutex<Cache<i64, String>>> = Lazy::new(|| {
    Mutex::new(Cache::new(10))
});

static CACHED_OVERIEW_REQUEST: Lazy<Mutex<Cache<i64, String>>> = Lazy::new(|| {
    Mutex::new(Cache::new(10))
});

static CACHED_RANKING_REQUEST: Lazy<Mutex<Cache<i64, String>>> = Lazy::new(|| {
    Mutex::new(Cache::new(10))
});
 
impl structs::UggRequest {
    pub async fn default_role(&self) -> Result<String, ErrorMap> {
        let cache = CACHED_DEFAULT_ROLE.lock().await;
        let role = cache.get(&self.id);
        if role != None {
            return Ok(role.unwrap())
        }
        let stat_version = "1.5";
        let base_role_url = "https://stats2.u.gg/lol";
        let role_version = "1.5.0";
        let future_data_dragon_version = data_dragon::structs::DataDragon::new(None);
        let client = &self.client;
        let (
            data_dragon_version, 
        ) = futures::join!(
            future_data_dragon_version, 
        );
        match data_dragon_version {
            Ok(data_dragon) => {
                let lol_version: Vec<&str> = data_dragon.version.split(".").collect();
                let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                let url = format!("{base_role_url}/{stat_version}/primary_roles/{ugg_lol_version}/{role_version}.json");
                let request = client.get(url).send().await;
                match request {
                    Ok(json) => {
                        let json: Result<HashMap<String, Vec<i64>>, reqwest::Error> = json.json().await;
                        match json {
                            Ok(json) => {
                                let role = &json[&self.id.to_string()][0].to_string();
                                cache.insert(self.id.clone(), role.to_string()).await;
                                cache.sync();
                                Ok(role.to_string())
                            },
                            Err(_) => Err(UGGError(UGGDataError::RoleMissing)),
                        }
                    }

                    Err(err) => {
                        if err.is_body() {
                            Err(UGGError(UGGDataError::RoleConnect))
                        } else {
                            Err(UGGError(UGGDataError::RoleMissing))
                        }
                    }
                }
            }
            Err(err) => Err(DataDragonErrors(err)),
        }
    }

    // Investigate wrapping https://stats2.u.gg/lol/1.5/ap-overview/12_20/ranked_solo_5x5/21/1.5.0.json
    // UPDATE: This is actually an easy drop in with the current system, but this is not offered to all champions.
    // Further investigation is needed into finding out which champs this is offered for automatically
    pub async fn overview_json(&self) -> Result<String, ErrorMap> {
        let cache = CACHED_OVERIEW_REQUEST.lock().await;
        let overview = cache.get(&self.id);
        if overview != None {
            return Ok(overview.unwrap())
        }
        let stats_version = "1.5";
        let overview_version = "1.5.0";
        let base_overview_url = "https://stats2.u.gg/lol";
        let game_mode = "ranked_solo_5x5";
    
        let future_data_dragon_version = data_dragon::structs::DataDragon::new(None);
        let client = &self.client;
        let (
            data_dragon_version, 
        ) = futures::join!(
            future_data_dragon_version, 
        );
    
        match data_dragon_version {
            Ok(data_dragon) => {
                let lol_version: Vec<&str> = data_dragon.version.split(".").collect();
                let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                let url = format!("{base_overview_url}/{stats_version}/overview/{ugg_lol_version}/{game_mode}/{0}/{overview_version}.json", self.id);
                let request = client.get(url).send().await;
                match request {
                    Ok(json) => {
                        let overview = json.text().await;
                        match overview {
                            Ok(valid) => {
                                let overview = valid.clone();
                                cache.insert(self.id.clone(), overview).await;
                                cache.sync();
                                Ok(valid)
                            },
                            Err(_) => Err(UGGError(UGGDataError::OverviewMissing)),
                        }
                    }
                    Err(err) => {
                        if err.is_connect() {
                            Err(UGGError(UGGDataError::OverviewConnect))
                        } else {
                            Err(UGGError(UGGDataError::OverviewMissing))
                        }
                    }
                }
            }
            Err(err) => Err(DataDragonErrors(err)),
        }
    }

    pub async fn ranking_json(&self) -> Result<String, ErrorMap> {
        let cache = CACHED_RANKING_REQUEST.lock().await;
        let ranking = cache.get(&self.id);
        if ranking != None {
            return Ok(ranking.unwrap())
        }
        let stats_version = "1.5";
        let overview_version = "1.5.0";
        let base_overview_url = "https://stats2.u.gg/lol";
        let game_mode = "ranked_solo_5x5";
    
        let future_data_dragon_version = data_dragon::structs::DataDragon::new(None);
        let client = &self.client;
        let (
            data_dragon_version, 
        ) = futures::join!(
            future_data_dragon_version, 
        );
    
        match data_dragon_version {
            Ok(data_dragon) => {
                let lol_version: Vec<&str> = data_dragon.version.split(".").collect();
                let ugg_lol_version = format!("{0}_{1}", lol_version[0], lol_version[1]);
                let url = format!("{base_overview_url}/{stats_version}/rankings/{ugg_lol_version}/{game_mode}/{0}/{overview_version}.json", self.id);
                let request = client.get(url).send().await;
                match request {
                    Ok(json) => {
                        let ranking = json.text().await;
                        match ranking {
                            Ok(valid) => {
                                let ranking = valid.clone();
                                cache.insert(self.id.clone(), ranking).await;
                                cache.sync();
                                Ok(valid)
                            },
                            Err(_) => Err(UGGError(UGGDataError::RankingMissing)),
                        }
                    }
                    Err(err) => {
                        if err.is_connect() {
                            Err(UGGError(UGGDataError::RankingConnect))
                        } else {
                            Err(UGGError(UGGDataError::RankingMissing))
                        }
                    }
                }
            }
            Err(err) => Err(DataDragonErrors(err)),
        }
    }
}
