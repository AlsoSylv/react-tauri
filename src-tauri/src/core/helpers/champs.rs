use crate::core::community_dragon::CommunityDragon;
use crate::errors::ErrorMap;
use crate::frontend_types::ChampionNames;
use data_dragon::DataDragon;

pub async fn get_champ_names(
    lang: &str,
    champions: &mut Vec<ChampionNames>,
    client: &reqwest::Client,
    data_dragon: &DataDragon<'_>,
) -> Result<(), ErrorMap> {
    match data_dragon.get_version().await {
        Ok(version) => match data_dragon.champion_json(&version).await {
            Ok(json) => {
                for (champ_key, champ) in json.data.iter() {
                    if let Ok(id) = champ.key.parse::<i64>() {
                        champions.push(ChampionNames::new(
                            &champ.name,
                            champ_key,
                            id,
                            Some(&version),
                        ));
                    } else {
                        unreachable!()
                    }
                }
                Ok(())
            }
            Err(err) => Err(ErrorMap::DataDragonErrors(err)),
        },
        Err(err) => {
            if err.is_connection() {
                Err(ErrorMap::DataDragonErrors(err))
            } else {
                let community_dragon = CommunityDragon::new(Some(lang), client);
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
                    }
                    Err(err) => Err(ErrorMap::CommunityDragonErrors(err)),
                }
            }
        }
    }
}
