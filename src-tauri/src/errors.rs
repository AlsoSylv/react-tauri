#[derive(Debug)]
pub enum DataDragonError {
    ChampMissingError,
    DataDragonMissing,
    CannotConnect,
}

impl From<DataDragonError> for i64 {
    fn from(error: DataDragonError) -> i64 {
        match error {
            DataDragonError::ChampMissingError => return 103,
            DataDragonError::DataDragonMissing => return 104,
            DataDragonError::CannotConnect => return 102,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum LCUResponses {
    LCUConnect,
    LCUDeleteRune,
    LCUCreateRune,
    LCUGetRune,
    LCUPushRune
}

impl From<LCUResponses> for i64 {
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

#[derive(Debug)]
pub enum ErrorMap {
    DataDragonErrors(DataDragonError),
    UGGError(UGGDataError),
    LCUReponse(LCUResponses)
}

impl From<ErrorMap> for i64 {
    fn from(error: ErrorMap) -> Self {
        match error {
            ErrorMap::DataDragonErrors(data_dragon) => return i64::from(data_dragon),
            ErrorMap::UGGError(ugg) => return i64::from(ugg),
            ErrorMap::LCUReponse(lcu) => return i64::from(lcu),
        }
    }
}