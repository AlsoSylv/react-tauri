use cached::proc_macro::cached;

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
            active: false }
    }
}

pub struct Data {
    pub name: String,
    pub role: String,
    pub rank: String,
    pub region: String,
}
