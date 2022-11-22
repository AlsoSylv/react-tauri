use serde_json::Value;

use crate::errors;

use errors::{ErrorMap, UGGDataError};

use super::{structs, constants};

use constants::STATS;

impl structs::Data {
    //The format is used here to get an exact result from the floating point math

    /// Returns the winrate from the UGG API, errors if it is None
    /// 
    /// ```rs
    /// format!("{:.1$}%", &win_rate * 100.0, 1)
    /// ```
    /// 
    /// This line is used to make sure that it's displayable information, 
    /// because returning it as a float leads to the number breaking easily
    pub async fn winrate(&self, request: Result<Value, ErrorMap>) -> Result<String, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(matches) = &json[STATS["matches"]].as_f64() else {
                    println!("FUCK");
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(wins) = &json[STATS["wins"]].as_f64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };

                let win_rate = wins / matches;
                Ok(format!("{:.1$}%", &win_rate * 100.0, 1))
            }
            Err(err) => {
                println!("FUCK");
                Err(err)
            }
        }
    }
    
    /// Returns the banrate from the UGG API, returns "-" if 
    /// bans is None, this follows UGGs pattern on their site
    /// 
    /// ```rs
    /// format!("{:.1$}%", &ban_rate * 100.0, 1)
    /// ```
    /// 
    /// This line is used to make sure that it's displayable information, 
    /// because returning it as a float leads to the number breaking easily
    pub async fn ban_rate(&self, request: Result<Value, ErrorMap>) -> Result<String, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(matches) = &json[STATS["total_matches"]].as_f64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(bans)= &json[STATS["bans"]].as_f64() else {
                    return Ok(format!("-"));
                };
                let ban_rate = bans / matches;
                Ok(format!("{:.1$}%", &ban_rate * 100.0, 1))
            }
            Err(err) => Err(err)
        }
    }

    /// Returns the pickrate from the UGG API, errors if it is None
    /// 
    /// ```rs
    /// format!("{:.1$}%", &pick_rate * 100.0, 1)
    /// ```
    /// 
    /// This line is used to make sure that it's displayable information, 
    /// because returning it as a float leads to the number breaking easily
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

    /// This currently returns the characters rank as an int,
    /// but this will change in the future and will be a string of 
    /// "{} / {}" rank, total_rank
    /// and will return a ? if it is not ranked
    pub async fn rank(&self, request: Result<Value, ErrorMap>) -> Result<i64, ErrorMap> {
        match request {
            Ok(json) => {
                //TODO: Return as rank/total-rank instead of just rank
                let Some(rank) = json[STATS["rank"]].as_i64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };
                Ok(rank)
            }
            Err(err) => Err(err)
        }
    }
}