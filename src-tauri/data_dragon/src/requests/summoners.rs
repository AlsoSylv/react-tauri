use crate::{DataDragon, types::Summoners, DataDragonError, request};

impl DataDragon {
    pub async fn summoners_json(&self) -> Result<Summoners, DataDragonError> {
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/runesReforged.json",
            &self.version, &self.language
        );
        let summoner_json = request::<Summoners, DataDragonError>(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        Ok(summoner_json)
    }
}