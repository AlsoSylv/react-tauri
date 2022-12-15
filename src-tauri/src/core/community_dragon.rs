/// Package concerning getting JSON from different champion endpoints
mod champs;
/// Package concerning getting JSON from different item endpoints
mod items;
/// Package for getting JSON from different rune endpoints
mod runes;
/// Structs to desearalize to
mod structs;

/// Struct for getting data from Community Dragon
///
/// This struct has a few methods, and is not meant to be used literally
///
/// # Examples
/// ```rs
/// use crate::core::community_dragon::CommunityDragon;
///
/// #[tokio::main]
/// async fn main() -> Result<(), CommunityDragonError> {
///     let community_dragon = CommunityDragon::new_with_client();
///     let champs_basic = community_dragon.champs_basic().await;
///     match champions_basic {
///         Ok(json) => { ... },
///         Err(community_dragon_error) => { ... },
///     }
/// }
pub struct CommunityDragon {
    pub language: String,
    pub client: reqwest::Client,
}

impl CommunityDragon {
    pub fn new(lang: &str) -> Self {
        let client = reqwest::Client::new();
        let binding = lang.to_lowercase();
        let language = match lang {
            "en_US" => "default",
            _ => &binding,
        }
        .to_owned();

        CommunityDragon { language, client }
    }
}
