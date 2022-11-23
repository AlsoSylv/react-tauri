use std::collections::HashMap;
use moka::future::{Cache, ConcurrentCacheExt};
use serde_json::Value;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

use crate::{core::data_dragon, extensions, errors};

use extensions::ugg::structs;
use errors::{ErrorMap, UGGDataError};
use ErrorMap::{DataDragonErrors, UGGError};

static CACHED_DEFAULT_ROLE: Lazy<Mutex<Cache<i64, String>>> = Lazy::new(|| {
    Mutex::new(Cache::new(10))
});

static CACHED_OVERIEW_REQUEST: Lazy<Mutex<Cache<i64, Value>>> = Lazy::new(|| {
    Mutex::new(Cache::new(10))
});

static CACHED_RANKING_REQUEST: Lazy<Mutex<Cache<i64, Value>>> = Lazy::new(|| {
    Mutex::new(Cache::new(10))
});
 
impl structs::UggRequest {
    /// Handles making the request to get the default roles for every champ 
    /// from the UGG api
    pub async fn default_role(&self) -> Result<String, ErrorMap> {
        let cache = CACHED_DEFAULT_ROLE.lock().await;
        if let Some(role) = cache.get(&self.id) {
            return Ok(role)
        };

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
                        if let Ok(json) = json.json::<HashMap<String, Vec<i64>>>().await {
                            let role = &json[&self.id.to_string()][0].to_string();
                            cache.insert(self.id.clone(), role.to_string()).await;
                            cache.sync();
                            Ok(role.to_string())
                        } else {
                            Err(UGGError(UGGDataError::RoleMissing))
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

    /// Handles making the network request for the UGG overview JSON file
    /// This contians things like rune IDs, item IDs, spell IDs, etc
    /// 
    /// 
    /// TODO: Investigate wrapping https://stats2.u.gg/lol/1.5/ap-overview/12_20/ranked_solo_5x5/21/1.5.0.json
    /// 
    /// UPDATE: This is actually an easy drop in with the current system, but this is not offered to all champions.
    /// Further investigation is needed into finding out which champs this is offered for automatically
    pub async fn overview_json(&self) -> Result<Value, ErrorMap> {
        let cache = CACHED_OVERIEW_REQUEST.lock().await;
        if let Some(overview) = cache.get(&self.id) {
            return Ok(overview)
        };
        
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
                        if let Ok(overview) = json.json::<Value>().await {
                            cache.insert(self.id.clone(), overview.clone()).await;
                            cache.sync();
                            Ok(overview)
                        } else {
                            Err(UGGError(UGGDataError::OverviewMissing))
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

    /// This handles making the request for the UGG ranking JSON for specific champs
    /// this contians things like pickrate, winrate, banrate, and matchups
    pub async fn ranking_json(&self) -> Result<Value, ErrorMap> {
        let cache = CACHED_RANKING_REQUEST.lock().await;
        if let Some(ranking) = cache.get(&self.id) {
            return Ok(ranking)
        };

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
                        if let Ok(ranking) = json.json::<Value>().await {
                            cache.insert(self.id.clone(), ranking.clone()).await;
                            cache.sync();
                            Ok(ranking)
                        } else {
                            Err(UGGError(UGGDataError::RankingMissing))
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
