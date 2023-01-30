use crate::{request, types::RuneTree, DataDragon, DataDragonError};

use moka::future::{Cache, ConcurrentCacheExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static CACHED_RUNE_JSON: Lazy<Mutex<Cache<String, Vec<RuneTree>>>> =
    Lazy::new(|| Mutex::new(Cache::new(3)));

impl DataDragon {
    /// Cached method to get the runesReforged.json from data dragon
    ///
    /// ```rust
    /// async fn runes_test() {
    ///     use data_dragon::DataDragon;
    ///
    ///     let data_dragon = DataDragon::new(None).await;
    ///     match data_dragon {
    ///         Ok(data_dragon) => {
    ///             let runes = data_dragon.rune_json().await;
    ///             match runes {
    ///                 Ok(json) => {
    ///                     let domination = &json[0];
    ///                     assert!(domination.id == 8100);
    ///                     assert!(domination.key == String::from("Domination"));
    ///
    ///                     let key_stones = &domination.slots[0].runes;
    ///                     assert!(key_stones[0].id == 8112);
    ///                     assert!(key_stones[0].key == String::from("Electrocute"));
    ///                 }
    ///                 Err(_) => panic!(),
    ///             }
    ///         }
    ///         Err(_) => panic!(),
    ///     }
    /// }
    /// ```
    pub async fn rune_json(&self) -> Result<Vec<RuneTree>, DataDragonError> {
        let cache = CACHED_RUNE_JSON.lock().await;
        if let Some(json) = cache.get(&self.language) {
            return Ok(json);
        };
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}/runesReforged.json",
            &self.version, &self.language
        );
        let rune_json: Vec<RuneTree> = request(
            &url,
            &self.client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        cache.insert(self.language.clone(), rune_json.clone()).await;
        cache.sync();
        Ok(rune_json)
    }
}
