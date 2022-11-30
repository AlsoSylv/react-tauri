mod champs;
mod items;
mod runes;
mod structs;

use moka::future::Cache;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::{errors::DataDragonError, templates::request};

/// A new struct for getting data from Data Dragon
pub struct DataDragon {
    pub version: String,
    pub language: String,
    pub client: reqwest::Client,
}

/// Version Cache
///
/// TODO: Figure out a timing system
/// Does not need to invalidate on lang change
static CACHED_VERSION: Lazy<Mutex<Cache<String, String>>> = Lazy::new(|| Mutex::new(Cache::new(1)));

impl DataDragon {
    /// A cached function to generate a new http client for Data Dragon
    /// this also gives a version string as a result, and can fail creation
    /// if there is no internet connection available
    ///
    /// # Examples
    /// ```
    /// let data_dragon = DataDragon::new(None).await;
    /// ```
    pub async fn new(language: Option<&str>) -> Result<Self, DataDragonError> {
        let lang = language.unwrap_or("en_US");

        let client = reqwest::Client::new();
        let cache = CACHED_VERSION.lock().await;
        if let Some(cache) = cache.get("version") {
            return Ok(DataDragon {
                version: cache,
                language: lang.to_string(),
                client,
            });
        }
        let url = "https://ddragon.leagueoflegends.com/api/versions.json";
        let request = request::<Vec<String>, DataDragonError>(url.to_owned(), &client, DataDragonError::DataDragonMissing, DataDragonError::CannotConnect).await;
        match request {
            Ok(json) => {
                let version = json[0].clone();
                cache.insert("version".to_string(), version.clone()).await;
                Ok(DataDragon {
                    version,
                    language: lang.to_string(),
                    client,
                })
            }
            Err(err) => Err(err),
        }
    }
}
