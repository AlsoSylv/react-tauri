use crate::core::{community_dragon::CommunityDragon, data_dragon::structs::DataDragon};
use crate::errors::{CommunityDragonError, DataDragonError};
use crate::frontend_types::ChampionNames;

pub async fn names_from_data_dragon(
    lang: &str,
    champions: &mut Vec<ChampionNames>,
) -> Result<(), DataDragonError> {
    let data_dragon = DataDragon::new(Some(lang)).await;
    match data_dragon {
        Ok(data_dragon) => {
            let champ_json = data_dragon.champion_json().await;
            match champ_json {
                Ok(json) => {
                    for (champ_key, champ) in json.data.iter() {
                        if let Ok(id) = champ.key.parse::<i64>() {
                            champions.push(ChampionNames::new(&champ.name, champ_key, id, None));
                        } else {
                            unreachable!()
                        }
                    }
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub async fn names_from_community_dragon(
    lang: &str,
    champions: &mut Vec<ChampionNames>,
) -> Result<(), CommunityDragonError> {
    let community_dragon = CommunityDragon::new_with_client(lang);
    let champ_json = community_dragon.champs_basic().await;
    match champ_json {
        Ok(json) => json.iter().for_each(|champ| {
            if champ.id > 0 {
                champions.push(ChampionNames::new(&champ.name, &champ.key, champ.id, None));
            }
        }),
        Err(err) => return Err(err),
    }
    Ok(())
}
