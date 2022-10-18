use cached::proc_macro::cached;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root2 {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<Slot>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub runes: Vec<Rune>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rune {
    pub id: i64,
    pub key: String,
    pub icon: String, 
    pub name: String,
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
}

#[cached(result = true)]
pub async fn data_dragon_version() -> Result<String, reqwest::Error> {
    let request: Result<Vec<String>, reqwest::Error> = reqwest::get("https://static.u.gg/assets/lol/riot_patch_update/prod/versions.json").await?.json().await;
    let version = &request.unwrap()[0];
    Ok(version.to_string())
}

#[cached(result = true)]
pub async fn runes_json() -> Result<Root, reqwest::Error> {
    let url = format!("https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/runesReforged.json", data_dragon_version().await.unwrap());
    let request: Result<Root, reqwest::Error> = reqwest::get(&url).await?.json().await;
    Ok(request.unwrap())
}

#[cached]
pub async fn champion_json() -> Value {
    let data_dragon_version = data_dragon_version().await.unwrap();
    let url = format!("https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json", data_dragon_version);
    let request: Result<String, reqwest::Error> = reqwest::get(url).await.unwrap().text().await;
    let champion_json: Value = serde_json::from_str(&request.unwrap()).unwrap();
    champion_json
}