use crate::frontend_types::ChampionNames;

mod abilities;
pub mod constants;
mod items;
pub mod json;
mod rates;
mod requests;
mod runes;
mod shards;
pub mod structs;

/// This is the Data struct for calling various methods from the UGG API
/// this handles things like getting champ winrates, pickrates, etc, and
/// should be used in order to reduce the amount of boilerplate garbage
pub struct Data {
    pub name: ChampionNames,
    pub role: String,
    pub rank: String,
    pub region: String,
    pub lang: String,
}

impl Data {
    /// Returns a new instance of the Data struct
    pub fn new(
        name: ChampionNames,
        role: String,
        rank: String,
        region: String,
        lang: String,
    ) -> Self {
        Data {
            name,
            role,
            rank,
            region,
            lang,
        }
    }
}
