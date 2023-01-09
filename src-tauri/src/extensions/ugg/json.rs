use serde_json::Value;

use crate::errors;

use errors::ErrorMap;

use super::{constants, structs, Data};

use constants::ROLES;
use structs::UggRequest;

impl Data {
    /// This handles accessing JSON for the champ, specifically for things like it's win rate
    /// this is important because it handles key checking, which will need to get more
    /// intense in the future
    pub async fn ranking(&self) -> Result<Value, ErrorMap> {
        let ugg = UggRequest::new(&self.name.value.id, &self.lang);
        let fut_request = ugg.ranking_json();
        let fut_role = position(&self.name.value.id, &self.role, &self.lang);
        let (request, role) = futures::join!(fut_request, fut_role);
        match request {
            Ok(ranking) => match role {
                Ok(role) => {
                    if let Some(json_read) = &ranking[&self.region] {
                        if let Some(json_read) = &json_read[&self.rank] {
                            if let Some(json_read) = &json_read[&role] {
                                Ok(json_read.clone())
                            } else {
                                Err(ErrorMap::UGGError(errors::UGGDataError::RoleHND))
                            }
                        } else {
                            Err(ErrorMap::UGGError(errors::UGGDataError::RankHND))
                        }
                    } else {
                        Err(ErrorMap::UGGError(errors::UGGDataError::RegionHND))
                    }
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    /// This handles accessing JSON for the champ, specifically for
    /// things such as runes and items, this is important because
    /// it handles error catching, which will get more intense
    pub async fn overview(&self) -> Result<Value, ErrorMap> {
        let ugg = UggRequest::new(&self.name.value.id, &self.lang);
        let fut_request = ugg.overview_json();
        let fut_role = position(&self.name.value.id, &self.role, &self.lang);
        let (request, role) = futures::join!(fut_request, fut_role);

        match request {
            Ok(overview) => match role {
                Ok(role) => {
                    if let Some(json_read) = &overview[&self.region] {
                        if let Some(json_read) = &json_read[&self.rank] {
                            if let Some(json_read) = &json_read[&role] {
                                // The zero is here because the only other data here is 
                                // The time that it was last updated, and u.gg doesn't
                                // show that data anyways
                                Ok(json_read[0].clone())
                            } else {
                                Err(ErrorMap::UGGError(errors::UGGDataError::RoleHND))
                            }
                        } else {
                            Err(ErrorMap::UGGError(errors::UGGDataError::RankHND))
                        }
                    } else {
                        Err(ErrorMap::UGGError(errors::UGGDataError::RegionHND))
                    }
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}

/// Gets the default position of the character as a string
async fn position(name: &i64, role: &str, lang: &str) -> Result<String, ErrorMap> {
    let ugg = UggRequest::new(name, lang);
    if role == "Default" {
        let role = ugg.default_role().await;
        match role {
            Ok(role) => Ok(role),
            Err(err) => Err(err),
        }
    } else {
        let role: &str = ROLES[role];
        Ok(role.to_string())
    }
}
