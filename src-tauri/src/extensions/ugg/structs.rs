use std::ops::Index;

use serde_json::Value;
/// Handles making a new reques for the UGG extension, this should be changed
/// to pass a barrowed reqwest client instead of using an owned reqwest client
pub struct UggRequest {
    pub id: i64,
    pub client: reqwest::Client,
    pub lang: String,
}

impl UggRequest {
    /// Returns a new UggRequest, this also handles spawning the HTTP client
    pub fn new(id: &i64, lang: &str) -> Self {
        let client = reqwest::Client::new();
        UggRequest {
            id: *id,
            client,
            lang: lang.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Regions<T> {
    #[serde(rename = "1")]
    pub north_america: Box<Option<Tiers<T>>>,
    #[serde(rename = "2")]
    pub eu_west: Box<Option<Tiers<T>>>,
    #[serde(rename = "3")]
    pub korea: Box<Option<Tiers<T>>>,
    #[serde(rename = "4")]
    pub eu_north: Box<Option<Tiers<T>>>,
    #[serde(rename = "5")]
    pub brazil: Box<Option<Tiers<T>>>,
    #[serde(rename = "6")]
    pub la_north: Box<Option<Tiers<T>>>,
    #[serde(rename = "7")]
    pub la_south: Box<Option<Tiers<T>>>,
    #[serde(rename = "8")]
    pub oce: Box<Option<Tiers<T>>>,
    #[serde(rename = "9")]
    pub russia: Box<Option<Tiers<T>>>,
    #[serde(rename = "10")]
    pub turkey: Box<Option<Tiers<T>>>,
    #[serde(rename = "11")]
    pub japan: Box<Option<Tiers<T>>>,
    #[serde(rename = "12")]
    pub world: Box<Option<Tiers<T>>>,
}

impl<T> Index<&str> for Regions<T> {
    type Output = Option<Tiers<T>>;

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
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Tiers<T> {
    #[serde(rename = "1")]
    pub challenger: Option<Roles<T>>,
    #[serde(rename = "2")]
    pub master: Option<Roles<T>>,
    #[serde(rename = "3")]
    pub diamond: Option<Roles<T>>,
    #[serde(rename = "4")]
    pub platinum: Option<Roles<T>>,
    #[serde(rename = "5")]
    pub gold: Option<Roles<T>>,
    #[serde(rename = "6")]
    pub silver: Option<Roles<T>>,
    #[serde(rename = "7")]
    pub bronze: Option<Roles<T>>,
    #[serde(rename = "8")]
    pub overall: Option<Roles<T>>,
    #[serde(rename = "10")]
    pub platinum_plus: Option<Roles<T>>,
    #[serde(rename = "11")]
    pub diamond_plus: Option<Roles<T>>,
    #[serde(rename = "12")]
    pub diamond_two_plus: Option<Roles<T>>,
    #[serde(rename = "13")]
    pub grandmaster: Option<Roles<T>>,
    #[serde(rename = "14")]
    pub master_plus: Option<Roles<T>>,
    #[serde(rename = "15")]
    pub iron: Option<Roles<T>>,
}

impl<T> Index<&str> for Tiers<T> {
    type Output = Option<Roles<T>>;

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
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Roles<T> {
    #[serde(rename = "4")]
    pub top: Option<T>,
    #[serde(rename = "1")]
    pub jungle: Option<T>,
    #[serde(rename = "5")]
    pub mid: Option<T>,
    #[serde(rename = "3")]
    pub adc: Option<T>,
    #[serde(rename = "2")]
    pub support: Option<T>,
}

impl<T> Index<&str> for Roles<T> {
    type Output = Option<T>;

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

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OverviewBase {
    pub overview: Overview,
    pub time: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Overview {
    pub perks: Value,
    pub summoner_spells: Value,
    pub starting_items: Value,
    pub mythic_and_core: Value,
    pub abilities: Value,
    pub other_items: Value,
    pub winrate: Value,
    pub _false: Value,
    pub shards: Value,
    pub _empty: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Ranking {
    pub wins: Value,
    pub matches: Value,
    pub rank: Value,
    pub total_rank: Value,
    _4: Value,
    _5: Value,
    _6: Value,
    _7: Value,
    _8: Value,
    _9: Value,
    pub bans: Value,
    pub total_matches: Value,
    pub matchups: Value,
    pub real_matches: Value,
    pub stdevs: Value,
    pub effective_winrate: Value,
    pub distribution_count: Value,
    pub distribution_mean: Value,
    pub distribution_stdevs: Value,
    pub be_all_picks: Value,
}
