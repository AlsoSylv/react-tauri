use crate::errors;

use errors::ErrorMap;

use super::{
    structs::{self, Overview, Ranking, JsonTypes},
    Data,
};

use structs::UggRequest;

impl Data {
    /// This handles accessing JSON for the champ, specifically for things like it's win rate
    /// this is important because it handles key checking, which will need to get more
    /// intense in the future
    pub async fn ranking(&self) -> Result<Ranking, ErrorMap> {
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
                                if let JsonTypes::Ranking(json) = json_read {
                                    Ok(json.to_owned())
                                } else {
                                    Err(ErrorMap::UGGError(errors::UGGDataError::RankingMissing))
                                }
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
    pub async fn overview(&self) -> Result<Overview, ErrorMap> {
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
                                if let JsonTypes::Overview(json) = &json_read {
                                    if let Some(json) = &json.overview {
                                        Ok(json.to_owned())
                                    } else {
                                        Err(ErrorMap::UGGError(errors::UGGDataError::OverviewMissing))
                                    }
                                } else {
                                    Err(ErrorMap::UGGError(errors::UGGDataError::OverviewMissing))
                                }
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
        let role: &str = match role {
            "Top" => "4",
            "Jungle" => "1",
            "Mid" => "5",
            "ADC" => "3",
            "Support" => "2",
            _ => unreachable!(),
        };
        Ok(role.to_string())
    }
}
