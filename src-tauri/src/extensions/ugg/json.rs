use serde_json::Value;

use crate::errors;

use errors::{ErrorMap, UGGDataError};
use ErrorMap::UGGError;

use super::{constants, structs};

use structs::UggRequest;
use constants::{TIERS, REGIONS, ROLES};

pub async fn ranking(
    name: &i64, 
    role: &str, 
    rank: &str, 
    region: &str,
    lang: &str,
) -> Result<Value, ErrorMap> {
    let ugg = UggRequest::new(name, lang);
    let fut_request = ugg.ranking_json();
    let fut_role = position(name, role, lang);
    let (request, role) = futures::join!(fut_request, fut_role);
    match request {
        Ok(ranking) => {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(&ranking);
            match json {
                Ok(json) => {
                    match role {
                        Ok(role) => {
                            //TODO: Check keys before reading, this can cause errors
                            let json_read: &Value = &json[REGIONS[&region]][TIERS[&rank]][&role];
                            Ok(json_read.to_owned())
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(_) => Err(UGGError(UGGDataError::RankingConnect)),
            }
        }
        Err(err) => Err(err),
    }
}

pub async fn overview(
    name: &i64,
    role: &str,
    rank: &str,
    region: &str,
    lang: &str,
) -> Result<Value, ErrorMap> {
    let ugg = UggRequest::new(name, lang);
    let fut_request = ugg.overview_json();
    let fut_role = position(name, role, lang);
    let (
        request, 
        role
    ) = futures::join!(
        fut_request, 
        fut_role
    );
    
    match request {
        Ok(overview) => {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(&overview);
            match json {
                Ok(json) => {
                    match role {
                        Ok(role) => {
                            //TODO: Check keys before reading, this can cause errors
                            let json_read: &Value = &json[REGIONS[&region]][TIERS[&rank]][&role][0];
                            Ok(json_read.to_owned())
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(_) => Err(UGGError(UGGDataError::OverviewConnect)),
            }
        }
        Err(err) => Err(err),
    }
}

async fn position(
    name: &i64, 
    role: &str,
    lang: &str,
) -> Result<String, ErrorMap> {
    let ugg = UggRequest::new(name, lang);
    if role == "Default" {
        let role = ugg.default_role().await;
        match role {
            Ok(role) => Ok(role),
            Err(err) => Err(err),
        }
    } else {
    let role = ROLES[&role];
        Ok(role.to_string())
    }
}
