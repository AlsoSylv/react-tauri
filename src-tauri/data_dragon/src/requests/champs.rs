use crate::{DataDragon, DataDragonError, types::{ChampJson, ChampionFull}, request};

impl DataDragon {
    pub async fn champion_json(&self) -> Result<ChampJson, DataDragonError> {
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion.json",
            &self.version, &self.language
        );
        let champ_json = request::<ChampJson, DataDragonError>(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        Ok(champ_json)
    }

    /// Method for getting the full json for a specific champion
    /// requires the key field be the same as the one that would
    /// be found for the champ in the champions.json file
    /// 
    /// # Example
    /// ```rust
    /// async fn champion_full() {
    ///     use data_dragon::DataDragon;
    /// 
    ///     let data_dragon = DataDragon::new(None).await;
    ///     match data_dragon {
    ///         Ok(data_dragon) => {
    ///             let json = data_dragon.champ_full("Xayah").await;
    ///             match json {
    ///                 Ok(json) => {
    ///                     if let Some(id) = json.data["Xayah"]["key"].as_str() {
    ///                         assert!(id == "498");
    ///                     } else {
    ///                         panic!()
    ///                     };
    ///                 }
    ///                 Err(_) => panic!(),
    ///             }
    ///         }
    ///         Err(_) => panic!(),
    ///     }
    /// }
    /// ```
    pub async fn champ_full(&self, key: &str) -> Result<ChampionFull, DataDragonError> {
        let url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion/{}.json",
            &self.version, &self.language, &key
        );
        let full_json = request::<ChampionFull, DataDragonError>(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        Ok(full_json)
    }
}