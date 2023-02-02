use crate::errors::CommunityDragonError;

use super::structs::Summoners;
use super::CommunityDragon;
use crate::templates::request;

impl CommunityDragon<'_> {
    pub async fn summoner_spells(&self) -> Result<Vec<Summoners>, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/perkstyles.json", &self.language);
        let request = request::<Vec<Summoners>, CommunityDragonError>(
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
