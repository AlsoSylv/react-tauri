/// A list of Data Dragon specific erreors with things like connections,
/// the champion being missing, or Data Dragon being missing
#[derive(Debug, Clone)]
pub enum DataDragonError {
    ChampMissingError,
    DataDragonMissing,
    CannotConnect,
}

impl From<DataDragonError> for i64 {
    /// Retuns the enum as an i64 for easy sending to the front end
    fn from(error: DataDragonError) -> i64 {
        match error {
            DataDragonError::ChampMissingError => return 103,
            DataDragonError::DataDragonMissing => return 104,
            DataDragonError::CannotConnect => return 102,
        }
    }
}

/// Returns sepcific errors for the UGG module, like connection, or
/// specific files being missing
#[derive(Debug, Clone)]
pub enum UGGDataError {
    OverviewMissing,
    OverviewConnect,
    RankingMissing,
    RankingConnect,
    RoleMissing,
    RoleConnect,
    NoAbilityOrder,
    RateError,
    MatchesError,
}

impl From<UGGDataError> for i64 {
    /// Retuns the enum as an i64 for easy sending to the front end
    fn from(error: UGGDataError) -> i64 {
        match error {
            UGGDataError::OverviewMissing => return 201,
            UGGDataError::OverviewConnect => return 202,
            UGGDataError::RankingMissing => return 203,
            UGGDataError::RankingConnect => return 204,
            UGGDataError::RateError => return 205,
            UGGDataError::MatchesError => return 206,
            UGGDataError::NoAbilityOrder => return 207,
            UGGDataError::RoleMissing => return 208,
            UGGDataError::RoleConnect => return 209,
        }
    }
}

/// Returns specfic errors for the LCU support, such as bing unable
/// to delete runes connect to the client, or push runes
#[derive(Debug, Clone)]
pub enum LCUResponses {
    LCUConnect,
    LCUDeleteRune,
    LCUCreateRune,
    LCUGetRune,
    LCUPushRune
}

impl From<LCUResponses> for i64 {
    /// Retuns the enum as an i64 for easy sending to the front end
    fn from(response: LCUResponses) -> i64 {
        match response {
            LCUResponses::LCUConnect => return 401,
            LCUResponses::LCUDeleteRune => return 402,
            LCUResponses::LCUCreateRune => return 403,
            LCUResponses::LCUGetRune => return 404,
            LCUResponses::LCUPushRune => return 405,
        }
    }
}

/// Wraps the existing errors inside of an error map to be able 
/// to pass them all the way down to the final tauri command
#[derive(Debug, Clone)]
pub enum ErrorMap {
    DataDragonErrors(DataDragonError),
    UGGError(UGGDataError),
    LCUReponse(LCUResponses)
}

impl From<ErrorMap> for i64 {
    /// Retuns the enum as an i64 for easy sending to the front end
    fn from(error: ErrorMap) -> Self {
        match error {
            ErrorMap::DataDragonErrors(data_dragon) => return i64::from(data_dragon),
            ErrorMap::UGGError(ugg) => return i64::from(ugg),
            ErrorMap::LCUReponse(lcu) => return i64::from(lcu),
        }
    }
}