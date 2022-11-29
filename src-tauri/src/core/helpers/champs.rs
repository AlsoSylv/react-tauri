use crate::frontend_types::{ChampionNames, ChampionValue};
use crate::errors::{DataDragonError, CommunityDragonError};
use crate::core::{data_dragon::structs::DataDragon, community_dragon::structs::CommunityDragon};

pub async fn names_from_data_dragon(lang: &str, champions: &mut Vec<ChampionNames>) -> Result<(), DataDragonError> {
    let data_dragon = DataDragon::new(Some(lang)).await;
    match data_dragon {
        Ok(data_dragon) => {
            let champ_json = data_dragon.champion_json().await;
            match champ_json {
                Ok(json) => {
                    for (champ_key, champ) in json.data.iter() {
                        if let Ok(id) = champ.key.parse::<i64>() {
                            champions.push(ChampionNames {
                                label: champ.name.clone(),
                                value: ChampionValue {
                                    key: champ_key.to_string(),
                                    id,
                                },
                                url: Some(format!(
                                  "https://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
                                  &data_dragon.version,
                                  &champ.id,
                              )),
                                local_image: Some(format!("/{0}/{0}.png", &champ.id)),
                            });
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

pub async fn names_from_community_dragon(lang: &str, champions: &mut Vec<ChampionNames>) -> Result<(), CommunityDragonError> {
    let community_dragon = CommunityDragon::new_with_client(lang);
    let champ_json = community_dragon.champs_basic().await;
    match champ_json {
        Ok(json) => {
            json.iter().for_each(|champ| {
                if champ.id > 0 {
                    champions.push(
                        ChampionNames { 
                        label: champ.name.clone(), 
                        value: ChampionValue { 
                            key: champ.key.clone(), 
                            id: champ.id 
                        }, 
                        url: Some(format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/{}.png", champ.id)), 
                        local_image: Some(format!("/{0}/{0}.png", champ.key))
                    });
                }
            })
        }
        Err(err) => return Err(err),
    }
    Ok(())  
}
