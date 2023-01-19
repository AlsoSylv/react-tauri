/// Package concerning getting JSON from different champion endpoints
mod champs;
/// Package concerning getting JSON from different item endpoints
mod items;
/// Package for getting JSON from different rune endpoints
mod runes;
/// Structs to desearalize to
pub mod structs;
/// Package for getting JSON from diffferent summoner spell endpoints
mod summoners;

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
///     let community_dragon: CommunityDragon = CommunityDragon::new();
///     let champs_basic: Result<Vec<ChampionData>, CommunityDragonError> = community_dragon.champs_basic().await;
///     match champions_basic {
///         Ok(json: Vec<ChampionData>) => { ... },
///         Err(community_dragon_error: CommunityDragonError) => { ... },
///     }
/// }
pub struct CommunityDragon {
    pub language: String,
    pub client: reqwest::Client,
}

impl CommunityDragon {
    /// Creates a new reqwest client for data dragon
    /// Takes a Riot language and translates it to
    /// A Community Dragon language, prefered over
    /// Using a literal struct.
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
