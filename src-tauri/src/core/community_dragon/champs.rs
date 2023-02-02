use super::structs::{ChampFull, ChampionData};
use super::CommunityDragon;
use crate::errors::CommunityDragonError;
use crate::templates;

impl CommunityDragon<'_> {
    pub async fn champs_basic(&self) -> Result<Vec<ChampionData>, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/champion-summary.json", &self.language);
        let request: Result<Vec<ChampionData>, CommunityDragonError> = templates::request(
            url,
            &self.client,
            CommunityDragonError::CommunityDragonMissing,
            CommunityDragonError::CannotConnect,
        )
        .await;
        match request {
            Ok(champion_json) => Ok(champion_json),
            Err(err) => Err(err),
        }
    }

    pub async fn champs_full(&self, id: i64) -> Result<ChampFull, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/champions/{}.json", &self.language, id);
        println!("{}", url);
        let request: Result<ChampFull, CommunityDragonError> = templates::request(
            url,
            &self.client,
            CommunityDragonError::CommunityDragonMissing,
            CommunityDragonError::CannotConnect,
        )
        .await;
        match request {
            Ok(champion_json) => Ok(champion_json),
            Err(err) => Err(err),
        }
    }
}
