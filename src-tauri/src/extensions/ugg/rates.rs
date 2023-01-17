use crate::errors;

use errors::{ErrorMap, UGGDataError};

use super::structs::Ranking;

impl super::Data {
    // The format is used here to get an exact result from the floating point math

    /// Returns the win rate from the UGG API, errors if it is None
    ///
    /// ```rs
    /// format!("{:.1$}%", &win_rate * 100.0, 1)
    /// ```
    ///
    /// This line is used to make sure that it's displayable information,
    /// because returning it as a float leads to the number breaking easily
    pub async fn winrate(&self, request: Result<Ranking, ErrorMap>) -> Result<String, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(matches) = json.matches else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(wins) = json.wins else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };

                let win_rate = wins / matches;
                Ok(format!("{:.1$}%", &win_rate * 100.0, 1))
            }
            Err(err) => Err(err),
        }
    }

    /// Returns the ban rate from the UGG API, returns "-" if
    /// bans is None, this follows UGGs pattern on their site
    ///
    /// ```rs
    /// format!("{:.1$}%", &ban_rate * 100.0, 1)
    /// ```
    ///
    /// This line is used to make sure that it's displayable information,
    /// because returning it as a float leads to the number breaking easily
    pub async fn ban_rate(&self, request: Result<Ranking, ErrorMap>) -> Result<String, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(matches) = &json.total_matches else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(bans)= &json.bans else {
                    return Ok("-".to_string());
                };
                let ban_rate = bans / matches;
                Ok(format!("{:.1$}%", &ban_rate * 100.0, 1))
            }
            Err(err) => Err(err),
        }
    }

    /// Returns the pick rate from the UGG API, errors if it is None
    ///
    /// ```rs
    /// format!("{:.1$}%", &pick_rate * 100.0, 1)
    /// ```
    ///
    /// This line is used to make sure that it's displayable information,
    /// because returning it as a float leads to the number breaking easily
    pub async fn pick_rate(&self, request: Result<Ranking, ErrorMap>) -> Result<String, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(matches) = &json.total_matches else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(picks) = &json.matches else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };

                let pick_rate = picks / matches;
                Ok(format!("{:.1$}%", &pick_rate * 100.0, 1))
            }
            Err(err) => Err(err),
        }
    }

    /// Returns the tier from the UGG API, errors if it is None
    pub async fn rank(&self, request: Result<Ranking, ErrorMap>) -> Result<String, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(rank) = json.rank else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };
                let Some(total_rank) = json.total_rank else {
                    return Err(ErrorMap::UGGError(UGGDataError::RateError));
                };

                Ok(format!("{} / {}", rank, total_rank))
            }
            Err(err) => Err(err),
        }
    }
}
