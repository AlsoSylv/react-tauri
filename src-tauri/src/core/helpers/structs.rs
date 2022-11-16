#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionNames {
    pub label: String,
    pub value: ChampionValue,
    pub url: String,
    pub local_image: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionValue {
    pub key: String,
    pub id: String,
}
