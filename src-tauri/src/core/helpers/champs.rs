use crate::{core::data_dragon::structs::DataDragon, errors::DataDragonError};

use super::structs::{ChampionNames, ChampionValue};

fn parse_id(id: &str) -> i64 {
    id.parse().unwrap()
}

pub async fn all_champion_names(lang: &str) -> Result<Vec<ChampionNames>, DataDragonError> {
    let mut champions = Vec::new();
    let data_dragon = DataDragon::new(Some(lang)).await;
    match data_dragon {
        Ok(data_dragon) => {
            let champ_json = data_dragon.champion_json().await;
            match champ_json {
                Ok(json) => {
                    for (champ_key, champ) in json.data.iter() {
                        let key = &champ.id;
                        champions.push(ChampionNames {
                          label: champ.clone().name,
                          value: ChampionValue { key: champ_key.to_string(), id: parse_id(&champ.key) },
                          url: format!(
                            "https://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
                            &data_dragon.version,
                            key,
                        ),
                          local_image: format!("/{0}/{0}.png", key),
                        });
                    }
                    Ok(champions)
                }
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    }
}

