use serde::Deserialize;

use crate::errors::CommunityDragonError;

use super::structs::{self, Runes, RunesStyle};

impl structs::CommunityDragon {
    pub async fn runes(&self) -> Result<Vec<Runes>, CommunityDragonError> {
        let url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/perks.json";
        let request = request::<Vec<Runes>>(url, &self.client).await;
        match request {
            Ok(valid) => Ok(valid),
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn runes_style(&self) -> Result<RunesStyle, CommunityDragonError> {
        let url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/perkstyles.json";
        let request = request::<RunesStyle>(url, &self.client).await;
        match request {
            Ok(valid) => Ok(valid),
            Err(err) => Err(err),
        }
    }
}

async fn request<T: for<'de> Deserialize<'de>>(
    url: &str,
    client: &reqwest::Client,
) -> Result<T, CommunityDragonError> {
    let request = client.get(url).send().await;
    match request {
        Ok(response) => {
            let Ok(json) = response.json::<T>().await else {
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
