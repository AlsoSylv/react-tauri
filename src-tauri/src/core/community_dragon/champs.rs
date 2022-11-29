use super::structs::{self, ChampionData, ChampionFull};
use super::templates;
use crate::errors::CommunityDragonError;

impl structs::CommunityDragon {
    pub async fn champs_basic(&self) -> Result<Vec<ChampionData>, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/champion-summary.json", &self.language);
        let request = templates::request::<Vec<ChampionData>>(url, &self.client).await;
        match request {
            Ok(champion_json) => Ok(champion_json),
            Err(err) => Err(err),
        }
    }

    pub async fn champs_full(&self, id: i64) -> Result<ChampionFull, CommunityDragonError> {
        let url = format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/champions/{}.json", &self.language, id);
        let request = templates::request::<ChampionFull>(url, &self.client).await;
        match request {
            Ok(champion_json) => Ok(champion_json),
            Err(err) => Err(err),
        }
    }
}