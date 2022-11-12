use super::structs::{self, ChampJson};

impl structs::DataDragon {
    pub async fn champion_json(&self) -> Result<ChampJson, i64> {
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion.json",
            &self.version,
            &self.language
        );
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let champ_json: Result<ChampJson, reqwest::Error> = response.json().await;
                match champ_json {
                    Ok(champ_json) => Ok(champ_json),
                    Err(_) => Err(103),
                }
            }
            Err(err) => {
                if err.is_body() {
                    Err(104)
                } else {
                    Err(103)
                }
            }
        }
    }
}