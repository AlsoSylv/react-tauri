use crate::errors;

use data_dragon::DataDragon;
use errors::ErrorMap;

use super::{
    structs::{new_ugg_request, JsonTypes, Overview, Ranking},
    Data,
};

impl Data<'_> {
    /// This handles accessing JSON for the champ, specifically for things like it's win rate
    /// this is important because it handles key checking, which will need to get more
    /// intense in the future
    pub async fn ranking(&self) -> Result<Ranking, ErrorMap> {
        let ugg = new_ugg_request(&self.name.value.id, self.client);
        let request = ugg.ranking_json(self.data_dragon).await;

        match request {
            Ok(ranking) => {
                if let Some(json_read) = &ranking[self.region] {
                    if let Some(json_read) = &json_read[self.rank] {
                        if let Some(json_read) = &json_read[self.role] {
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
        let ugg = new_ugg_request(&self.name.value.id, self.client);
        let request = ugg.overview_json(self.data_dragon).await;

        match request {
            Ok(overview) => {
                if let Some(json_read) = &overview[self.region] {
                    if let Some(json_read) = &json_read[self.rank] {
                        if let Some(json_read) = &json_read[self.role] {
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
        let ugg = new_ugg_request(&self.name.value.id, self.client);
        let role = ugg.default_role(self.data_dragon).await;
        match role {
            Ok(role) => Ok(role),
            Err(err) => Err(err),
        }
    }

    pub async fn no_pos(
        id: i64,
        client: &reqwest::Client,
        data_dragon: &DataDragon<'_>,
    ) -> Result<String, ErrorMap> {
        let ugg = new_ugg_request(&id, client);
        let role = ugg.default_role(data_dragon).await;
        match role {
            Ok(role) => Ok(role),
            Err(err) => Err(err),
        }
    }
}
