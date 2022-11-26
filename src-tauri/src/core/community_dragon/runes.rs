use crate::errors::CommunityDragonError;

use super::structs::{self, Runes};

impl structs::CommunityDragon {
    pub async fn runes(&self) -> Result<Vec<Runes>, CommunityDragonError>{
        let url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/perks.json";
        let request = self.client.get(url).send().await;
        match request {
            Ok(response) => {
                let Ok(json) = response.json::<Vec<Runes>>().await else {
                    todo!()
                };
                Ok(json)
            }
            Err(err) => {
                if err.is_body() {
                    todo!()
                } else {
                    todo!()
                }
            }
        }
    }
}