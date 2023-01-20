#![allow(dead_code)]
/// A list of Data Dragon specific errors with things like connections,
/// the champion being missing, or Data Dragon being missing

pub trait Errors {
    fn is_connection(&self) -> bool;

    fn is_missing(&self) -> bool;

    fn is_champ_missing(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommunityDragonError {
    CommunityDragonMissing = 105,
    ChampMissingError = 103,
    CannotConnect = 102,
}

impl Errors for CommunityDragonError {
    fn is_connection(&self) -> bool {
        self == &CommunityDragonError::CannotConnect
    }

    fn is_missing(&self) -> bool {
        self == &CommunityDragonError::CommunityDragonMissing
    }

    fn is_champ_missing(&self) -> bool {
        self == &CommunityDragonError::ChampMissingError
    }
}

/// Returns specific errors for the UGG module, like connection, or
/// specific files being missing
#[derive(Debug, Clone, PartialEq, Eq)]
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
    RegionHND = 210,
    RankHND = 211,
    RoleHND = 212,
}

/// Returns specific errors for the LCU support, such as bing unable
/// to delete runes connect to the client, or push runes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LCUResponses {
    LCUConnect = 401,
    LCUDeleteRune = 402,
    LCUCreateRune = 403,
    LCUGetRune = 404,
    LCUPushRune = 405,
    LCUGetSummoner = 406,
    LCUGetItems = 407,
    LCUItemsPushedMaybe = 408,
}

/// Wraps the existing errors inside of an error map to be able
/// to pass them all the way down to the final Tauri command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorMap {
    DataDragonErrors(data_dragon::DataDragonError),
    UGGError(UGGDataError),
    LCUResponse(LCUResponses),
    CommunityDragonErrors(CommunityDragonError),
}

impl From<ErrorMap> for i64 {
    /// Returns the enum as an i64 for easy sending to the front end
    fn from(error: ErrorMap) -> Self {
        match error {
            ErrorMap::DataDragonErrors(data_dragon) => data_dragon as i64,
            ErrorMap::UGGError(ugg) => ugg as i64,
            ErrorMap::LCUResponse(lcu) => lcu as i64,
            ErrorMap::CommunityDragonErrors(community_dragon) => community_dragon as i64,
        }
    }
}
