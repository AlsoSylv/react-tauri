use crate::core::{community_dragon::CommunityDragon, data_dragon::DataDragon};
use crate::errors::{ErrorMap, Errors};
use crate::frontend_types::ChampionNames;

pub async fn get_champ_names(
    lang: &str,
    champions: &mut Vec<ChampionNames>,
) -> Result<(), ErrorMap> {
    match DataDragon::new(Some(lang)).await {
        Ok(data_dragon) => match data_dragon.champion_json().await {
            Ok(json) => {
                for (champ_key, champ) in json.data.iter() {
                    if let Ok(id) = champ.key.parse::<i64>() {
                        champions.push(ChampionNames::new(
                            &champ.name,
                            champ_key,
                            id,
                            Some(&data_dragon.version),
                        ));
                    } else {
                        unreachable!()
                    }
                };
                Ok(())
            },
            Err(err) => Err(ErrorMap::DataDragonErrors(err)),
        },
        Err(err) => {
            if err.is_connection() {
                Err(ErrorMap::DataDragonErrors(err))
            } else {
                let community_dragon = CommunityDragon::new(lang);
                match community_dragon.champs_basic().await {
                    Ok(json) => {
                        json.iter().for_each(|champ| {
                            if champ.id > 0 {
                                champions.push(ChampionNames::new(
                                    &champ.name,
                                    &champ.key,
                                    champ.id,
                                    None,
                                ));
                            }
                        });
                        Ok(())
                    },
                    Err(err) => Err(ErrorMap::CommunityDragonErrors(err)),
                }
            }
        }
    }
}
