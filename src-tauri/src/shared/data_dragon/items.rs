use serde_json::Value;

use super::structs;

impl structs::DataDragon {
    pub async fn item_json(&self) -> Result<Value, i64> {
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/item.json",
            &self.version,
            &self.language
        );
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let item_json: Result<Value, reqwest::Error> = response.json().await;
                match item_json {
                    Ok(item_json) => Ok(item_json),
                    Err(_) => Err(103),
                }
            },
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