use crate::errors::CommunityDragonError;

use super::structs::{Runes, RunesStyle};
use super::CommunityDragon;
use crate::templates::request;

impl CommunityDragon<'_> {
    pub async fn runes(&self) -> Result<Vec<Runes>, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/perks.json", &self.language);
        let request = request::<Vec<Runes>, CommunityDragonError>(
            url,
            &self.client,
            CommunityDragonError::CommunityDragonMissing,
            CommunityDragonError::CannotConnect,
        )
        .await;
        match request {
            Ok(valid) => Ok(valid),
            Err(err) => Err(err),
        }
    }

    pub async fn runes_style(&self) -> Result<RunesStyle, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/perkstyles.json", &self.language);
        let request = request::<RunesStyle, CommunityDragonError>(
            url,
            &self.client,
            CommunityDragonError::CommunityDragonMissing,
            CommunityDragonError::CannotConnect,
        )
        .await;
        match request {
            Ok(valid) => Ok(valid),
            Err(err) => Err(err),
        }
    }
}
