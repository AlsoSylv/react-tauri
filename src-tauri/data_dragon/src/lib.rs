use hyper::{
    client::{connect::Connect, HttpConnector},
    Client,
};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use moka::future::Cache;

#[path ="requests/champs.rs"]
mod champs;
#[path ="requests/runes.rs"]
mod runes;
#[path ="requests/items.rs"]
mod items;
#[path ="requests/summoners.rs"]
mod summoners;

pub mod types;

static CACHED_VERSION: Lazy<Mutex<Cache<String, String>>> = Lazy::new(|| Mutex::new(Cache::new(1)));

pub struct DataDragon {
    pub version: String,
    pub language: String,
    pub client: Client<HttpsConnector<HttpConnector>>,
}

impl DataDragon {
    /// Creates a new instance of the DataDragon wrapper
    /// 
    /// ```rust
    /// async fn new_test() {
    ///     use data_dragon::DataDragon;
    /// 
    ///     let data_dragon = DataDragon::new(None).await;
    ///     match data_dragon {
    ///         Ok(data_dragon) => {
    ///             println!("{}", data_dragon.version);
    ///             assert!(data_dragon.language == String::from("en_US"));
    ///         }
    ///         Err(_) => panic!(),
    ///     }
    /// }
    /// ```
    pub async fn new(language: Option<&str>) -> Result<Self, DataDragonError> {
        let lang = language.unwrap_or("en_US");
        let https = HttpsConnector::new();
        let client = Client::builder().build::<HttpsConnector<HttpConnector>, hyper::Body>(https);
        let cache = CACHED_VERSION.lock().await;
        if let Some(version) = cache.get(lang) {
            return Ok(DataDragon {
                version,
                language: lang.to_owned(),
                client,
            });
        }
        let json: Vec<String> = request(
            "https://ddragon.leagueoflegends.com/api/versions.json",
            &client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        let version = json[0].clone();
        cache.insert("version".to_string(), version.clone()).await;
        Ok(DataDragon {
            version,
            language: lang.to_owned(),
            client,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataDragonError {
    ChampMissingError = 103,
    DataDragonMissing = 104,
    CannotConnect = 102,
}

impl DataDragonError {
    pub fn is_connection(&self) -> bool {
        self == &DataDragonError::CannotConnect
    }

    pub fn is_champ_missing(&self) -> bool {
        self == &DataDragonError::ChampMissingError
    }

    pub fn is_missing(&self) -> bool {
        self == &DataDragonError::DataDragonMissing
    }
}

pub async fn request<
    T: for<'de> Deserialize<'de>,
    E,
    U: Connect + Send + Sync + Clone + 'static,
>(
    url: &str,
    client: &Client<U, hyper::Body>,
    error_one: E,
    error_two: E,
) -> Result<T, E> {
    let uri = url.parse().unwrap();
    match client.get(uri).await {
        Ok(mut resp) => {
            if resp.status().is_client_error() || resp.status().is_server_error() {
                return Err(error_two);
            }
            let body = resp.body_mut();
            let bytes = hyper::body::to_bytes(body).await;
            match bytes {
                Ok(bytes) => {
                    let json = serde_json::from_slice::<T>(&bytes);
                    match json {
                        Ok(json) => Ok(json),
                        Err(_) => Err(error_one),
                    }
                }
                Err(_) => Err(error_one),
            }
        }
        Err(err) => {
            if err.is_connect() {
                Err(error_two)
            } else {
                Err(error_one)
            }
        }
    }
}