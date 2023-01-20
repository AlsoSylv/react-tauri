use serde::Deserialize;

#[path ="requests/champs.rs"]
mod champs;
#[path ="requests/runes.rs"]
mod runes;
#[path ="requests/items.rs"]
mod items;
#[path ="requests/summoners.rs"]
mod summoners;

pub mod types;

pub struct DataDragon {
    pub version: String,
    pub language: String,
    pub client: reqwest::Client,
}

impl DataDragon {
    pub async fn new(language: Option<&str>) -> Result<Self, DataDragonError> {
        let lang = language.unwrap_or("en_US");
        let client = reqwest::Client::new();
        let json = request::<Vec<String>, DataDragonError>(
            "https://ddragon.leagueoflegends.com/api/versions.json",
            &client,
            DataDragonError::DataDragonMissing,
            DataDragonError::CannotConnect,
        )
        .await?;
        let version = json[0].clone();
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

async fn request<T: for<'de> Deserialize<'de>, E>(
    url: &str,
    client: &reqwest::Client,
    error_one: E,
    error_two: E,
) -> Result<T, E> {
    match client.get(url).send().await {
        Ok(response) => response.json::<T>().await.map_err(|_| error_one),
        Err(err) => {
            if err.is_body() {
                Err(error_one)
            } else {
                Err(error_two)
            }
        }
    }
}
