use std::ops::Index;

use phf::{phf_map, phf_ordered_map};
use serde_json::Value;

// These are used in the U.GG JSON to map the value to the human readable name
// This is done for the purpose of code readability, as well as sanity.

// TODO: These need to support translations somehow, ideally automatically

/// A list of different regions for LoL in the form of an ordered map to send to the FE
/// this will be changed in the future to be a function that returns a list based
/// on the currently selected language
pub static REGIONS: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "World" => "12",
    "North America" => "1",
    "EU West" => "2",
    "EU North" => "4",
    "Korea" => "3",
    "Brazil" => "5",
    "LA North" => "6",
    "LA South" => "7",
    "OCE" => "8",
    "Russia" => "9",
    "Turkey" => "10",
    "Japan" => "11",
};

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Regions {
    #[serde(rename = "1")]
    pub north_america: Box<Option<Tiers>>,
    #[serde(rename = "2")]
    pub eu_west: Box<Option<Tiers>>,
    #[serde(rename = "3")]
    pub korea: Box<Option<Tiers>>,
    #[serde(rename = "4")]
    pub eu_north: Box<Option<Tiers>>,
    #[serde(rename = "5")]
    pub brazil: Box<Option<Tiers>>,
    #[serde(rename = "6")]
    pub la_north: Box<Option<Tiers>>,
    #[serde(rename = "7")]
    pub la_south: Box<Option<Tiers>>,
    #[serde(rename = "8")]
    pub oce: Box<Option<Tiers>>,
    #[serde(rename = "9")]
    pub russia: Box<Option<Tiers>>,
    #[serde(rename = "10")]
    pub turkey: Box<Option<Tiers>>,
    #[serde(rename = "11")]
    pub japan: Box<Option<Tiers>>,
    #[serde(rename = "12")]
    pub world: Box<Option<Tiers>>,
}

impl Index<&str> for Regions {
    type Output = Option<Tiers>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "North America" => &self.north_america,
            "EU West" => &self.eu_west,
            "Korea" => &self.korea,
            "EU North" => &self.eu_north,
            "Brazil" => &self.brazil,
            "LA North" => &self.la_north,
            "LA South" => &self.la_south,
            "OCE" => &self.oce,
            "Russia" => &self.russia,
            "Turkey" => &self.turkey,
            "Japan" => &self.japan,
            "World" => &self.world,
            _ => unreachable!()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Tiers {
    #[serde(rename = "1")]
    pub challenger: Option<Roles>,
    #[serde(rename = "2")]
    pub master: Option<Roles>,
    #[serde(rename = "3")]
    pub diamond: Option<Roles>,
    #[serde(rename = "4")]
    pub platinum: Option<Roles>,
    #[serde(rename = "5")]
    pub gold: Option<Roles>,
    #[serde(rename = "6")]
    pub silver: Option<Roles>,
    #[serde(rename = "7")]
    pub bronze: Option<Roles>,
    #[serde(rename = "8")]
    pub overall: Option<Roles>,
    #[serde(rename = "10")]
    pub platinum_plus: Option<Roles>,
    #[serde(rename = "11")]
    pub diamond_plus: Option<Roles>,
    #[serde(rename = "12")]
    pub diamond_two_plus: Option<Roles>,
    #[serde(rename = "13")]
    pub grandmaster: Option<Roles>,
    #[serde(rename = "14")]
    pub master_plus: Option<Roles>,
    #[serde(rename = "15")]
    pub iron: Option<Roles>,
}

impl Index<&str> for Tiers {
    type Output = Option<Roles>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "Challenger" => &self.challenger,
            "Master" => &self.master,
            "Diamond" => &self.diamond,
            "Platinum" => &self.platinum,
            "Gold" => &self.gold,
            "Silver" => &self.silver,
            "Bronze" => &self.bronze,
            "Iron" => &self.iron,
            "Overall" => &self.overall,
            "Master Plus" => &self.master_plus,
            "Diamond Plus" => &self.diamond_plus,
            "Diamond 2 Plus" => &self.diamond_two_plus,
            "Platinum Plus" => &self.platinum_plus,
            _ => unreachable!()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Roles {
    #[serde(rename = "4")]
    pub top: Option<Value>,
    #[serde(rename = "1")]
    pub jungle: Option<Value>,
    #[serde(rename = "5")]
    pub mid: Option<Value>,
    #[serde(rename = "3")]
    pub adc: Option<Value>,
    #[serde(rename = "2")]
    pub support: Option<Value>
}

impl Index<&str> for Roles {
    type Output = Option<Value>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "4" => &self.top,
            "1" => &self.jungle,
            "5" => &self.mid,
            "3" => &self.adc,
            "2" => &self.support,
            _ => unreachable!(),
        }
    }
}

/// A list of different ranks for LoL in the form of an ordered map to send to the FE
/// this will be changed in the future to be a function that returns a list based
/// on the currently selected language
pub static TIERS: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "Challenger" => "1",
    "Grandmaster" => "13",
    "Master" => "2",
    "Diamond" => "3",
    "Platinum" => "4",
    "Gold" => "5",
    "Silver" => "6",
    "Bronze" => "7",
    "Iron" => "15",
    "Overall" => "8",
    "Master Plus" => "14",
    "Diamond Plus" => "11",
    "Diamond 2 Plus" => "12",
    "Platinum Plus" => "10",
};

/// A list of different roles for LoL to send to the FE
/// this will be changed in the future to send images
/// and use the number/name as a value system
pub static ROLES: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "Top" => "4",
    "Jungle" => "1",
    "Mid" => "5",
    "ADC" => "3",
    "Support" => "2",
};

/// Internal constant for commonly used parts of the UGG JSON files
pub static DATA: phf::Map<&'static str, usize> = phf_map! {
    "perks" => 0,
    "summoner_spells" => 1,
    "starting_items" => 2,
    "mythic_and_core" => 3,
    "abilities" => 4,
    "other_items" => 5,
    "shards" => 8,
};

/// Internal constant for commonly used parts of the UGG JSON files
pub static STATS: phf::Map<&'static str, usize> = phf_map! {
    "wins" => 0,
    "matches" => 1,
    "rank" => 2,
    "total_rank" => 3,
    // ?????????????
    "bans" => 10,
    "total_matches" => 11,
    "matchups" => 12, /* 2D Array, [0] = champion_id, [1] = loses, [2] = matches */
    "real_matches" => 13,
    "stdevs" => 14,
    "effective_winrate" => 15,
    "distribution_count" => 16,
    "distribution_mean" => 17,
    "distribution_stdevs" => 18,
    "be_all_picks" => 19,
};
