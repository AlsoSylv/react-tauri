use data_dragon::DataDragon;
use moka::future::{Cache, ConcurrentCacheExt};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::templates::request;
use crate::{errors, extensions};

use errors::{ErrorMap, UGGDataError};
use extensions::ugg::structs;
use ErrorMap::DataDragonErrors;

use super::structs::Regions;

static CACHED_DEFAULT_ROLE: Lazy<Mutex<HashMap<String, Vec<i64>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static CACHED_OVERIEW_REQUEST: Lazy<Mutex<Cache<i64, Regions>>> =
    Lazy::new(|| Mutex::new(Cache::new(10)));

static CACHED_RANKING_REQUEST: Lazy<Mutex<Cache<i64, Regions>>> =
    Lazy::new(|| Mutex::new(Cache::new(10)));

impl structs::UggRequest<'_> {
    /// Handles making the request to get the default roles for every champ
    /// from the UGG api
    pub async fn default_role(&self, data_dragon: &DataDragon<'_>) -> Result<String, ErrorMap> {
        let mut cache = CACHED_DEFAULT_ROLE.lock().await;
        if let Some(role) = cache.get(&self.id.to_string()) {
            let role = role[0].to_string();
            return Ok(role);
        };

        let stat_version = "1.5";
        let role_version = "1.5.0";
        let base_role_url = "https://stats2.u.gg/lol";
        let lol_version = lol_version(data_dragon).await;
        let client = &self.client;
        match lol_version {
            Ok(ugg_lol_version) => {
                let url = format!("{base_role_url}/{stat_version}/primary_roles/{ugg_lol_version}/{role_version}.json");
                let role_json = request::<HashMap<String, Vec<i64>>, UGGDataError>(
                    url,
                    client,
                    UGGDataError::RoleMissing,
                    UGGDataError::RoleConnect,
                )
                .await;

                match role_json {
                    Ok(json) => {
                        let role = &json[&self.id.to_string()][0].to_string();
                        cache.extend(json.into_iter());
                        Ok(role.to_string())
                    }

                    Err(err) => Err(ErrorMap::UGGError(err)),
                }
            }
            Err(err) => Err(err),
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
    pub async fn overview_json(&self, data_dragon: &DataDragon<'_>) -> Result<Regions, ErrorMap> {
        let cache = CACHED_OVERIEW_REQUEST.lock().await;
        if let Some(overview) = cache.get(&self.id) {
            return Ok(overview);
        };

        let stats_version = "1.5";
        let overview_version = "1.5.0";
        let base_overview_url = "https://stats2.u.gg/lol";
        let game_mode = "ranked_solo_5x5";
        let lol_version = lol_version(data_dragon).await;
        let client = &self.client;

        match lol_version {
            Ok(ugg_lol_version) => {
                let url = format!("{base_overview_url}/{stats_version}/overview/{ugg_lol_version}/{game_mode}/{0}/{overview_version}.json", self.id);
                let overview = request::<Regions, UGGDataError>(
                    url,
                    client,
                    UGGDataError::OverviewMissing,
                    UGGDataError::OverviewConnect,
                )
                .await;

                match overview {
                    Ok(json) => {
                        // println!("{:?}", json);
                        cache.insert(self.id, json.clone()).await;
                        cache.sync();
                        Ok(json)
                    }
                    Err(err) => Err(ErrorMap::UGGError(err)),
                }
            }
            Err(err) => Err(err),
        }
    }

    /// This handles making the request for the UGG ranking JSON for specific champs
    /// this contians things like pickrate, winrate, banrate, and matchups
    pub async fn ranking_json(&self, data_dragon: &DataDragon<'_>) -> Result<Regions, ErrorMap> {
        let cache = CACHED_RANKING_REQUEST.lock().await;
        if let Some(ranking) = cache.get(&self.id) {
            return Ok(ranking);
        };

        let stats_version = "1.5";
        let overview_version = "1.5.0";
        let base_overview_url = "https://stats2.u.gg/lol";
        let game_mode = "ranked_solo_5x5";

        let lol_version = lol_version(data_dragon).await;
        let client = &self.client;

        match lol_version {
            Ok(ugg_lol_version) => {
                let url = format!("{base_overview_url}/{stats_version}/rankings/{ugg_lol_version}/{game_mode}/{0}/{overview_version}.json", self.id);
                let ranking = request::<Regions, UGGDataError>(
                    url,
                    client,
                    UGGDataError::RankingMissing,
                    UGGDataError::RankingConnect,
                )
                .await;

                match ranking {
                    Ok(json) => {
                        cache.insert(self.id, json.clone()).await;
                        cache.sync();
                        Ok(json)
                    }
                    Err(err) => Err(ErrorMap::UGGError(err)),
                }
            }
            Err(err) => Err(err),
        }
    }
}

/// This returns the ugg lol version, this removes a ton of duplicated code
async fn lol_version(data_dragon: &DataDragon<'_>) -> Result<String, ErrorMap> {
    match data_dragon.get_version().await {
        Ok(version) => {
            let lol_version: Vec<&str> = version.split('.').collect();
            Ok(format!("{0}_{1}", lol_version[0], lol_version[1]))
        }
        Err(err) => Err(DataDragonErrors(err)),
    }
}
