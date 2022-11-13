use super::structs::{self, RuneTree};

impl structs::DataDragon {
    pub async fn runes_json(&self) -> Result<Vec<RuneTree>, i64> {
        let data_dragon_version = &self.version;
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/runesReforged.json", 
            data_dragon_version,
            &self.language
        );
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let rune_json: Result<Vec<RuneTree>, reqwest::Error> = response.json().await;
                match rune_json {
                    Ok(rune_json) => Ok(rune_json),
                    Err(_) => Err(104),
                }
            }
            Err(_) => Err(104),
        }
    }
}