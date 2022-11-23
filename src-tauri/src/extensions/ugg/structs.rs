use crate::frontend_types::ChampionNames;

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
        lang: String
    ) -> Self {
        return Data { 
            name, 
            role, 
            rank, 
            region,
            lang: lang.to_string(),
        }
    }
}

pub struct UggRequest {
    pub id: i64,
    pub client: reqwest::Client,
    pub lang: String,
}

impl UggRequest {
    /// Returns a new UggRequest, this also handles spawning the HTTP client
    pub fn new(id: &i64, lang: &str) -> Self {
        let client = reqwest::Client::new();
        return UggRequest { id: *id, client, lang: lang.to_string() }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ItemsMap {
    pub start: Vec<ItemValues>,
    pub core: Vec<ItemValues>,
    pub fourth: Vec<ItemValues>,
    pub fifth: Vec<ItemValues>,
    pub sixth: Vec<ItemValues>
}

impl ItemsMap {
    /// Returns a new instance of the ItemsMap struct
    pub fn new() -> Self {
        return ItemsMap {             
            start: Vec::new(), 
            core: Vec::new(), 
            fourth: Vec::new(), 
            fifth: Vec::new(), 
            sixth: Vec::new()  
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    pub fn new(
        name: &str, 
        cost: i64, 
        description: &str, 
        image: &str, 
        url: &str
    ) -> Self {
        return ItemValues { 
            name: name.to_owned(), 
            cost, 
            description: description.to_owned(), 
            local_image: image.to_owned(), 
            url: url.to_owned()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    /// Returns q, w, e, r as a mutable array
    pub fn as_array_mut(&mut self) -> [&mut AbilitiesValue; 4] {
        return [
            &mut self.q,
            &mut self.w,
            &mut self.e,
            &mut self.r
        ];
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AbilitiesValue {
    pub name: String,
    pub image: String,
    pub order: Vec<String>,
    pub url: String,
}

impl AbilitiesValue {
    /// Returns a new instance of the AbilitiesValue struct
    pub fn new(name: &str, image: &str, url: String) -> Self {
        return AbilitiesValue { 
            name: name.to_owned(),
            image: image.to_owned(), 
            order: Vec::new(), 
            url
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Passive {
    pub image: String,
    pub url: String,
}

impl Passive {
    /// Returns a new instance of the Passive struct
    pub fn new(image: &str, url: String ) -> Self {
        return Passive { 
            image: image.to_owned(), 
            url 
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
        return [
            &mut self.row_one,
            &mut self.row_two,
            &mut self.row_three,
        ];
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shard {
    pub name: String,
    pub id: i64,
    pub image: String,
    pub active: bool,
}

impl Shard {
    /// Returns a new instance of the Shard struct
    pub fn create(
        name: &str,
        id: i64,
        image: &str,
    ) -> Shard {
        return Shard { 
            name: name.to_string(), 
            id, 
            image: image.to_string(), 
            active: false 
        }
    }
}
