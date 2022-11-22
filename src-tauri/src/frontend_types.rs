use crate::extensions::ugg::structs::{ItemsMap, AbilitiesMap, Shards};

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionInfo {
    pub url: String,
    pub local_image: String,
    pub win_rate: String,
    pub pick_rate: String,
    pub ban_rate: String,
    pub tier: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneImages {
    pub primary_runes: PrimaryTree,
    pub secondary_runes: SecondaryTree,
}

impl RuneImages {
    /// Requires the intial struct to be mutable.
    /// 
    /// Returns slots from PrimaryTree and SecondaryTree
    /// as an array of mutabily barrowed vectors
    pub fn as_array_mut(&mut self) -> [&mut Vec<Active>; 7] {
        return [
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

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
        return PrimaryTree { 
            slot_one: Vec::new(), 
            slot_two: Vec::new(), 
            slot_three: Vec::new(),
            slot_four: Vec::new(),
        }
    }

    /// Requires the intial struct to be mutable.
    /// 
    /// Returns slots SecondaryTree as an array of mutabily barrowed vectors
    pub fn as_array_mut(&mut self) -> [&mut Vec<Active>; 4] {
        return [
            &mut self.slot_one,
            &mut self.slot_two,
            &mut self.slot_three,
            &mut self.slot_four,
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryTree {
    pub slot_one: Vec<Active>,
    pub slot_two: Vec<Active>,
    pub slot_three: Vec<Active>,
}

impl SecondaryTree {
    /// Creates a new PrimaryTree struct
    pub fn new() -> Self {
        return SecondaryTree { 
            slot_one: Vec::new(), 
            slot_two: Vec::new(), 
            slot_three: Vec::new() 
        }
    }

    /// Requires the intial struct to be mutable.
    /// 
    /// Returns slots SecondaryTree as an array of mutabily barrowed vectors
pub fn as_array_mut(&mut self) -> [&mut Vec<Active>; 3] {
        return [
            &mut self.slot_one,
            &mut self.slot_two,
            &mut self.slot_three,
        ]
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    pub fn new(
        name: &str, 
        image: String, 
        id: i64, 
        local_image: String, 
        description: &str
    ) -> Self {
        return Active { 
            name: name.to_owned(), 
            image,
            local_image, 
            active: false, 
            id,
            description: description.to_owned() 
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunesAndAbilities {
    pub runes: RuneImages,
    pub items: ItemsMap,
    pub abilities: AbilitiesMap,
    pub shards: Shards,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionNames {
    pub label: String,
    pub value: ChampionValue,
    pub url: Option<String>,
    pub local_image: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionValue {
    pub key: String,
    pub id: i64,
}

