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
            _ => unreachable!(),
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
            _ => unreachable!(),
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
    pub support: Option<Value>,
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
