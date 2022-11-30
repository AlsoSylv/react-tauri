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

/// Returns the different item sets in the form of a JSON map for the frontend
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemsMap {
    pub start: Vec<ItemValues>,
    pub core: Vec<ItemValues>,
    pub fourth: Vec<ItemValues>,
    pub fifth: Vec<ItemValues>,
    pub sixth: Vec<ItemValues>,
}

impl ItemsMap {
    /// Returns a new instance of the ItemsMap struct
    pub fn new() -> Self {
        ItemsMap {
            start: Vec::new(),
            core: Vec::new(),
            fourth: Vec::new(),
            fifth: Vec::new(),
            sixth: Vec::new(),
        }
    }

    pub fn as_array_mut(&mut self) -> [&mut Vec<ItemValues>; 5] {
        [
            &mut self.start,
            &mut self.core,
            &mut self.fourth,
            &mut self.fifth,
            &mut self.sixth,
        ]
    }
}

/// This returns an items value struct containing things like cost and description
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemValues {
    pub name: String,
    pub cost: i64,
    pub description: String,
    pub local_image: String,
    pub url: String,
}

impl ItemValues {
    /// Returns a new instance of the ItemValues struct
    pub fn new(name: &str, cost: i64, description: &str, image: &str, url: &str) -> Self {
        ItemValues {
            name: name.to_owned(),
            cost,
            description: description.to_owned(),
            local_image: image.to_owned(),
            url: url.to_owned(),
        }
    }
}

/// Abilities map is a struct that contains the passive as well as the abilities
/// for a specific champion.
///
/// When calling things like as_array_mut(), it will only return Q, W, E, and R
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AbilitiesMap {
    pub passive: Passive,
    pub q: AbilitiesValue,
    pub w: AbilitiesValue,
    pub e: AbilitiesValue,
    pub r: AbilitiesValue,
}

impl AbilitiesMap {
    /// Requires the intial struct to be mutable
    ///
    /// Returns Q, W, E, R as a mutable array
    pub fn as_array_mut(&mut self) -> [&mut AbilitiesValue; 4] {
        [&mut self.q, &mut self.w, &mut self.e, &mut self.r]
    }
}

/// Returns an abilities value struct, with the order in which you level that specific ability
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AbilitiesValue {
    pub name: String,
    pub image: String,
    pub order: Vec<String>,
    pub url: String,
}

impl AbilitiesValue {
    /// Returns a new instance of the AbilitiesValue struct
    pub fn new(name: &str, image: &str, url: String) -> Self {
        AbilitiesValue {
            name: name.to_owned(),
            image: image.to_owned(),
            order: Vec::new(),
            url,
        }
    }
}

// Returns the data for the passive
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Passive {
    pub image: String,
    pub url: String,
}

impl Passive {
    /// Returns a new instance of the Passive struct
    pub fn new(image: &str, url: String) -> Self {
        Passive {
            image: image.to_owned(),
            url,
        }
    }
}

/// Returns a map of arrays of Shards
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shards {
    pub row_one: [Shard; 3],
    pub row_two: [Shard; 3],
    pub row_three: [Shard; 3],
}

impl Shards {
    /// Requires that the intial struct also be mutable
    ///
    /// This returns the Shards struct as an array of
    /// &mut arrays of Shards
    pub fn as_array_mut(&mut self) -> [&mut [Shard; 3]; 3] {
        [&mut self.row_one, &mut self.row_two, &mut self.row_three]
    }
}

/// Returns the data for specific shards
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Shard {
    pub name: String,
    pub id: i64,
    pub image: String,
    pub active: bool,
    pub description: String,
}

impl Shard {
    /// Returns a new instance of the Shard struct
    pub fn new(id: i64, image: &str) -> Shard {
        let base_url = "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods";
        Shard {
            name: "".to_string(),
            id,
            image: format!("{}/{}", &base_url, image),
            active: false,
            description: "".to_string(),
        }
    }
}
