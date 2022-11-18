use serde_json::Value;

use crate::errors;

use errors::{ErrorMap, UGGDataError};

use super::{structs, constants};

use constants::STATS;

impl structs::Data {
    //The format is used here to get an exact result from the floating point math
    pub async fn winrate(&self, request: Result<Value, ErrorMap>) -> Result<String, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(matches) = &json[STATS["matches"]].as_f64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(wins) = &json[STATS["wins"]].as_f64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };

                let win_rate = wins / matches;
                Ok(format!("{:.1$}%", &win_rate * 100.0, 1))
            }
            Err(err) => Err(err)
        }
    }
    
    pub async fn ban_rate(&self, request: Result<Value, ErrorMap>) -> Result<String, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(matches) = &json[STATS["total_matches"]].as_f64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(bans)= &json[STATS["bans"]].as_f64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };
                let ban_rate = bans / matches;
                Ok(format!("{:.1$}%", &ban_rate * 100.0, 1))
            }
            Err(err) => Err(err)
        }
    }

    pub async fn pick_rate(&self, request: Result<Value, ErrorMap>) -> Result<String, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(matches) = &json[STATS["total_matches"]].as_f64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(picks) = &json[STATS["matches"]].as_f64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };

                let pick_rate = picks / matches;
                Ok(format!("{:.1$}%", &pick_rate * 100.0, 1))
            }
            Err(err) => Err(err)
        }
    }

    pub async fn rank(&self, request: Result<Value, ErrorMap>) -> Result<i64, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(rank) = json[STATS["rank"]].as_i64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };
                Ok(rank)
            }
            Err(err) => Err(err)
        }
    }
}