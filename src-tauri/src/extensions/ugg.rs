use data_dragon::DataDragon;

use crate::frontend_types::ChampionNames;

mod abilities;
mod items;
mod json;
mod rates;
mod requests;
mod runes;
mod shards;
mod structs;
mod summoners;

/// This is the Data struct for calling various methods from the UGG API
/// this handles things like getting champ winrates, pickrates, etc, and
/// should be used in order to reduce the amount of boilerplate garbage
pub struct Data<'a> {
    pub name: ChampionNames,
    pub role: String,
    pub rank: String,
    pub region: String,
    pub lang: Option<&'a str>,
    data_dragon: &'a DataDragon,
    client: &'a reqwest::Client,
}

/// Returns a new instance of the Data struct
pub fn data_new<'a>(
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: Option<&'a str>,
    data_dragon: &'a DataDragon,
    client: &'a reqwest::Client,
) -> Data<'a> {
    Data {
        name,
        role,
        rank,
        region,
        lang,
        data_dragon,
        client,
    }
}
