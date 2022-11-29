use crate::errors::CommunityDragonError;

use super::structs::{self, Runes, RunesStyle};
use super::templates::request;

impl structs::CommunityDragon {
    pub async fn runes(&self) -> Result<Vec<Runes>, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/perks.json", &self.language);
        let request = request::<Vec<Runes>>(url, &self.client).await;
        match request {
            Ok(valid) => Ok(valid),
            Err(err) => Err(err),
        }
    }

    pub async fn runes_style(&self) -> Result<RunesStyle, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/perkstyles.json", &self.language);
        let request = request::<RunesStyle>(url, &self.client).await;
        match request {
            Ok(valid) => Ok(valid),
            Err(err) => Err(err),
        }
    }
}
