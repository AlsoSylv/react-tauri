use crate::{DataDragon, types::RuneTree, DataDragonError, request};

impl DataDragon {
    pub async fn rune_json(&self) -> Result<Vec<RuneTree>, DataDragonError> {
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/runesReforged.json",
            &self.version, &self.language
        );
        let rune_json = request::<Vec<RuneTree>, DataDragonError>(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        Ok(rune_json)
    }
}