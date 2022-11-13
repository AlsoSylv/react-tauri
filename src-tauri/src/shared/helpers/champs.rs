use crate::shared::data_dragon::structs::DataDragon;

use super::structs::ChampionNames;

pub async fn champion_id(name: &str) -> Result<i64, i64> {
    let champion_name = format!("{}", name.clone());
    let data_dragon = DataDragon::new(Some("en_US".to_string())).await;
    match data_dragon {
        Ok(data_dragon) => {
            let request = data_dragon.champion_json().await;
            match request {
                Ok(json) => Ok(json.data[&champion_name].key.parse().unwrap()),
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    }
}

pub async fn all_champion_names() -> Result<Vec<ChampionNames>, i64> {
    let mut champions = Vec::new();
    let data_dragon = DataDragon::new(Some("en_US".to_string())).await;
    match data_dragon {
        Ok(data_dragon) => {
            let champ_json = data_dragon.champion_json().await;
            match champ_json {
                Ok(json) => {
                    for (champ_key, champ) in json.data.iter() {
                        let key = &champ.id;
                        champions.push(ChampionNames {
                          label: champ.clone().name,
                          value: champ_key.to_string(),
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

