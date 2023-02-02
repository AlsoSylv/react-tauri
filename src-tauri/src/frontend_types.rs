/// Frontend type for packing data from the ranking JSON into a map
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionInfo {
    pub url: Result<String, i64>,
    pub local_image: String,
    pub win_rate: Result<String, i64>,
    pub pick_rate: Result<String, i64>,
    pub ban_rate: Result<String, i64>,
    pub tier: Result<String, i64>,
    pub role: Result<String, i64>,
    pub runes: Result<RuneImages, i64>,
    pub items: Result<ItemsMap, i64>,
    pub abilities: Result<AbilitiesMap, i64>,
    pub shards: Result<Shards, i64>,
    pub spells: Result<SummonerSpellInfo, i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LCUItemsMap {
    pub start: Vec<LCUItemsValue>,
    pub core: Vec<LCUItemsValue>,
    pub fourth: Vec<LCUItemsValue>,
    pub fifth: Vec<LCUItemsValue>,
    pub sixth: Vec<LCUItemsValue>,
}

impl LCUItemsMap {
    pub fn new() -> Self {
        LCUItemsMap {
            start: Vec::new(),
            core: Vec::new(),
            fourth: Vec::new(),
            fifth: Vec::new(),
            sixth: Vec::new(),
        }
    }

    pub fn as_array_mut(&mut self) -> [&mut Vec<LCUItemsValue>; 5] {
        [
            &mut self.start,
            &mut self.core,
            &mut self.fourth,
            &mut self.fifth,
            &mut self.sixth,
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LCUItemsValue {
    pub id: String,
    pub count: i64,
}

impl LCUItemsValue {
    pub fn new(id: &str) -> Self {
        LCUItemsValue {
            id: id.to_owned(),
            count: 1,
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

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpellInfo {
    pub spell_one: Spell,
    pub spell_two: Spell,
    pub winrate: String,
}

impl SummonerSpellInfo {
    pub fn new(winrate: String) -> Self {
        SummonerSpellInfo {
            spell_one: Default::default(),
            spell_two: Default::default(),
            winrate,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    pub name: String,
    pub description: String,
    pub url: String,
    pub local_image: String,
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
    pub image: Option<String>,
    pub order: Vec<String>,
    pub url: String,
}

impl AbilitiesValue {
    /// Returns a new instance of the AbilitiesValue struct
    pub fn new(name: &str, image: &str, url: String) -> Self {
        AbilitiesValue {
            name: name.to_owned(),
            image: Some(image.to_owned()),
            order: Vec::new(),
            url,
        }
    }

    /// The current fallback system does not work with Community
    /// Dragon, so the image is null
    pub fn new_cd(name: &str, url: String) -> Self {
        AbilitiesValue {
            name: name.to_owned(),
            image: None,
            order: Vec::new(),
            url,
        }
    }
}

// Returns the data for the passive
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Passive {
    pub image: Option<String>,
    pub url: String,
}

impl Passive {
    /// Returns a new instance of the Passive struct
    pub fn new(image: &str, url: String) -> Self {
        Passive {
            image: Some(image.to_owned()),
            url,
        }
    }

    /// The current fallback system does not work with Community
    /// Dragon, so the image is null
    pub fn new_cd(url: String) -> Self {
        Passive { image: None, url }
    }
}

/// Map to display runes in the same way that U.GG and the LoL client do
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneImages {
    pub tree_one: i64,
    pub tree_two: i64,
    pub primary_runes: PrimaryTree,
    pub secondary_runes: SecondaryTree,
}

impl RuneImages {
    /// Requires the initial struct to be mutable.
    ///
    /// Returns slots from PrimaryTree and SecondaryTree
    /// as an array of mutably borrowed vectors
    pub fn as_array_mut(&mut self) -> [&mut Vec<Active>; 7] {
        [
            &mut self.primary_runes.slot_one,
            &mut self.primary_runes.slot_two,
            &mut self.primary_runes.slot_three,
            &mut self.primary_runes.slot_four,
            &mut self.secondary_runes.slot_one,
            &mut self.secondary_runes.slot_two,
            &mut self.secondary_runes.slot_three,
        ]
    }
}

/// A structure for the primary rune tree selected
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryTree {
    pub slot_one: Vec<Active>,
    pub slot_two: Vec<Active>,
    pub slot_three: Vec<Active>,
    pub slot_four: Vec<Active>,
}

impl PrimaryTree {
    /// Creates a new PrimaryTree struct
    pub fn new() -> Self {
        PrimaryTree {
            slot_one: Vec::new(),
            slot_two: Vec::new(),
            slot_three: Vec::new(),
            slot_four: Vec::new(),
        }
    }

    /// Requires the initial struct to be mutable.
    ///
    /// Returns slots PrimaryTree as an array of mutably borrowed vectors
    pub fn as_array_mut(&mut self) -> [&mut Vec<Active>; 4] {
        [
            &mut self.slot_one,
            &mut self.slot_two,
            &mut self.slot_three,
            &mut self.slot_four,
        ]
    }
}

/// A structure for the secondary rune tree selected
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryTree {
    pub slot_one: Vec<Active>,
    pub slot_two: Vec<Active>,
    pub slot_three: Vec<Active>,
}

impl SecondaryTree {
    /// Creates a new PrimaryTree struct
    pub fn new() -> Self {
        SecondaryTree {
            slot_one: Vec::new(),
            slot_two: Vec::new(),
            slot_three: Vec::new(),
        }
    }

    /// Requires the initial struct to be mutable.
    ///
    /// Returns slots SecondaryTree as an array of mutably borrowed vectors
    pub fn as_array_mut(&mut self) -> [&mut Vec<Active>; 3] {
        [&mut self.slot_one, &mut self.slot_two, &mut self.slot_three]
    }
}

/// A structure for all active runes
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Active {
    pub name: String,
    pub image: String,
    pub local_image: String,
    pub active: bool,
    pub id: i64,
    pub description: String,
}

impl Active {
    pub fn new(name: &str, image: String, id: i64, local_image: String, description: &str) -> Self {
        Active {
            name: name.to_owned(),
            image,
            local_image,
            active: false,
            id,
            description: description.to_owned(),
        }
    }
}

/// Struct for packing runes, items, abilities, etc, into a JSON map for the frontend
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunesAndAbilities {
    pub runes: Result<RuneImages, i64>,
    pub items: Result<ItemsMap, i64>,
    pub abilities: Result<AbilitiesMap, i64>,
    pub shards: Result<Shards, i64>,
    pub spells: Result<SummonerSpellInfo, i64>,
}

/// Struct for passing champion names, and champ values into a JSON map for the frontend
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionNames {
    pub label: String,
    pub value: ChampionValue,
    pub url: Option<String>,
    pub local_image: Option<String>,
}

impl ChampionNames {
    pub fn new(label: &str, key: &str, id: i64, version: Option<&str>) -> Self {
        let url = match version {
            Some(version) => format!("https://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png", version, key),
            None => format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/{}.png", id),
        };
        ChampionNames {
            label: label.to_owned(),
            value: ChampionValue {
                key: key.to_owned(),
                id,
            },
            url: Some(url),
            local_image: Some(format!("/{0}/{0}.png", key)),
        }
    }
}

/// A map containing the champions Key and Id
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionValue {
    pub key: String,
    pub id: i64,
}
