use serde_json::Value;

use super::{constants::{TIERS, REGIONS, ROLES}, structs::UggRequest};

pub async fn ranking(
    name: String, 
    role: String, 
    ranks: String, 
    regions: String
) -> Result<Value, i64> {
    let ugg = UggRequest::new(name.clone());
    let fut_request = ugg.ranking_json();
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

pub async fn overview(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<Value, i64> {
    let ugg = UggRequest::new(name.clone());
    let fut_request = ugg.overview_json();
    let fut_role = position(name, role);
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

async fn position(name: String, role: String) -> Result<String, i64> {
    let ugg = UggRequest::new(name);
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
