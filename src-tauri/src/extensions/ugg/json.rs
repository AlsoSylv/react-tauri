use crate::errors;

use errors::ErrorMap;

use super::{
    structs::{self, JsonTypes, Overview, Ranking},
    Data,
};

use structs::UggRequest;

impl Data {
    /// This handles accessing JSON for the champ, specifically for things like it's win rate
    /// this is important because it handles key checking, which will need to get more
    /// intense in the future
    pub async fn ranking(&self) -> Result<Ranking, ErrorMap> {
        let ugg = UggRequest::new(&self.name.value.id, &self.lang);
        let request = ugg.ranking_json().await;

        match request {
            Ok(ranking) => {
                if let Some(json_read) = &ranking[&self.region] {
                    if let Some(json_read) = &json_read[&self.rank] {
                        if let Some(json_read) = &json_read[&self.role] {
                            if let JsonTypes::Ranking(json) = json_read {
                                Ok(*json.to_owned())
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
        }
    }

    /// This handles accessing JSON for the champ, specifically for
    /// things such as runes and items, this is important because
    /// it handles error catching, which will get more intense
    pub async fn overview(&self) -> Result<Overview, ErrorMap> {
        let ugg = UggRequest::new(&self.name.value.id, &self.lang);
        let request = ugg.overview_json().await;

        match request {
            Ok(overview) => {
                if let Some(json_read) = &overview[&self.region] {
                    if let Some(json_read) = &json_read[&self.rank] {
                        if let Some(json_read) = &json_read[&self.role] {
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
        }
    }

    pub async fn default_pos(&self) -> Result<String, ErrorMap> {
        let ugg = UggRequest::new(&self.name.value.id, &self.lang);
        let role = ugg.default_role().await;
        match role {
            Ok(role) => Ok(role),
            Err(err) => Err(err),
        }
    }

    pub async fn no_pos(id: i64, lang: &str) -> Result<String, ErrorMap> {
        let ugg = UggRequest::new(&id, lang);
        let role = ugg.default_role().await;
        match role {
            Ok(role) => Ok(role),
            Err(err) => Err(err),
        }
    }
}
