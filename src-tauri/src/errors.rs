/// A list of Data Dragon specific erreors with things like connections,
/// the champion being missing, or Data Dragon being missing
#[derive(Debug, Clone)]
pub enum DataDragonError {
    ChampMissingError = 103,
    DataDragonMissing = 104,
    CannotConnect = 102,
}

/// Returns sepcific errors for the UGG module, like connection, or
/// specific files being missing
#[derive(Debug, Clone)]
pub enum UGGDataError {
    OverviewMissing = 201,
    OverviewConnect = 202,
    RankingMissing = 203,
    RankingConnect = 204,
    RoleMissing = 208,
    RoleConnect = 209,
    NoAbilityOrder = 207,
    RateError = 205,
    MatchesError = 206,
}

/// Returns specfic errors for the LCU support, such as bing unable
/// to delete runes connect to the client, or push runes
#[derive(Debug, Clone)]
pub enum LCUResponses {
    LCUConnect = 401,
    LCUDeleteRune = 402,
    LCUCreateRune = 403,
    LCUGetRune = 404,
    LCUPushRune = 405,
}

/// Wraps the existing errors inside of an error map to be able
/// to pass them all the way down to the final tauri command
#[derive(Debug, Clone)]
pub enum ErrorMap {
    DataDragonErrors(DataDragonError),
    UGGError(UGGDataError),
    LCUReponse(LCUResponses),
}

impl From<ErrorMap> for i64 {
    /// Retuns the enum as an i64 for easy sending to the front end
    fn from(error: ErrorMap) -> Self {
        return match error {
            ErrorMap::DataDragonErrors(data_dragon) => data_dragon as i64,
            ErrorMap::UGGError(ugg) => ugg as i64,
            ErrorMap::LCUReponse(lcu) => lcu as i64,
        };
    }
}
