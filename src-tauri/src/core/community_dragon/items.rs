use crate::errors::CommunityDragonError;

use super::structs::Items;
use super::CommunityDragon;
use crate::templates::request;

impl CommunityDragon<'_> {
    pub async fn item_json(&self) -> Result<Vec<Items>, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/items.json", &self.language);
        let request: Result<Vec<Items>, CommunityDragonError> = request(
            url,
            &self.client,
            CommunityDragonError::CommunityDragonMissing,
            CommunityDragonError::CannotConnect,
        )
        .await;
        match request {
            Ok(item_json) => Ok(item_json),
            Err(err) => Err(err),
        }
    }
}
