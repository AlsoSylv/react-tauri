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

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
            _ => &None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
            _ => &None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Roles {
    #[serde(rename = "4")]
    pub top: Option<JsonTypes>,
    #[serde(rename = "1")]
    pub jungle: Option<JsonTypes>,
    #[serde(rename = "5")]
    pub mid: Option<JsonTypes>,
    #[serde(rename = "3")]
    pub adc: Option<JsonTypes>,
    #[serde(rename = "2")]
    pub support: Option<JsonTypes>,
}

impl Index<&str> for Roles {
    type Output = Option<JsonTypes>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "4" => &self.top,
            "1" => &self.jungle,
            "5" => &self.mid,
            "3" => &self.adc,
            "2" => &self.support,
            _ => &None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum JsonTypes {
    Overview(OverviewBase),
    Ranking(Ranking),
}

impl Default for JsonTypes {
    fn default() -> Self {
        Self::Overview(OverviewBase::default())
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OverviewBase {
    pub overview: Option<Overview>,
    pub time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Overview {
    pub perks: Perks,
    pub summoner_spells: SummonerSpells,
    pub starting_items: CoreItems,
    pub mythic_and_core: CoreItems,
    pub abilities: Value,
    pub other_items: Vec<Vec<Items>>,
    pub winrate: Option<Winrate>,
    _false: bool,
    pub shards: Option<Shards>,
    _empty: Vec<i8>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Perks {
    pub matches: Option<f64>,
    pub wins: Option<f64>,
    pub tree_one_id: Option<i64>,
    pub tree_two_id: Option<i64>,
    pub rune_ids: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SummonerSpells {
    pub matches: Option<f64>,
    pub wins: Option<f64>,
    pub spells: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoreItems {
    pub matches: Option<f64>,
    pub wins: Option<f64>,
    pub ids: Option<Vec<Option<i64>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Items {
    pub id: Option<i64>,
    pub wins: Option<i64>,
    pub matches: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Winrate {
    pub wins: Option<f64>,
    pub matches: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Shards {
    pub matches: i64,
    pub wins: i64,
    pub shards: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Ranking {
    pub wins: Option<f64>,
    pub matches: Option<f64>,
    pub rank: Option<i64>,
    pub total_rank: Option<i64>,
    _4: i64,
    _5: i64,
    _6: i64,
    _7: i64,
    _8: i64,
    _9: i64,
    pub bans: Option<f64>,
    pub total_matches: Option<f64>,
    pub matchups: Vec<Vec<i64>>,
    pub real_matches: Option<f64>,
    _stdevs: f64,
    _effective_winrate: f64,
    _distribution_count: i64,
    _distribution_mean: f64,
    _distribution_stdevs: f64,
    _be_all_picks: i64,
}
