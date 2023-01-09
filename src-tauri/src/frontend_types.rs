use crate::extensions::ugg::structs::{AbilitiesMap, ItemsMap, Shards, SummonerSpellInfo};

/// Frontend type for packing data from the ranking JSON into a map
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionInfo {
    pub url: String,
    pub local_image: String,
    pub win_rate: Result<String, i64>,
    pub pick_rate: Result<String, i64>,
    pub ban_rate: Result<String, i64>,
    pub tier: Result<String, i64>,
}

/// Map to display runes in the same way that U.GG and the LoL client do
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneImages {
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
