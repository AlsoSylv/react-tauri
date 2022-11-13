pub struct Data {
    pub name: String,
    pub role: String,
    pub rank: String,
    pub region: String,
    pub lang: String,
}

impl Data {
    pub fn new(name: String, role: String, rank: String, region: String) -> Self {
        let lang = "en_US".to_string();
        return Data { 
            name, 
            role, 
            rank, 
            region,
            lang,
        }
    }
}

pub struct UggRequest {
    pub name: String,
    pub client: reqwest::Client,
}

impl UggRequest {
    pub fn new(name: String) -> Self {
        let client = reqwest::Client::new();
        return UggRequest { name, client }
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

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemValues {
    pub name: String,
    pub cost: String,
    pub description: String,
    pub local_image: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AbilitiesMap {
    pub passive: Passive,
    pub q: AbilitiesValue,
    pub w: AbilitiesValue,
    pub e: AbilitiesValue,
    pub r: AbilitiesValue,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AbilitiesValue {
    pub image: String,
    pub order: Vec<String>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Passive {
    pub image: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shards {
    pub row_one: [Shard; 3],
    pub row_two: [Shard; 3],
    pub row_three: [Shard; 3],
}


#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shard {
    pub name: String,
    pub id: i64,
    pub image: String,
    pub active: bool,
}

impl Shard {
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
