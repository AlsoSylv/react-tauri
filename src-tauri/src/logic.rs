use crate::{frontend_types::ChampionInfo, plugins::ugg::structs::Data, shared::data_dragon::structs::DataDragon};

pub async fn champion_info(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<ChampionInfo, i64> {
    let data_dragon = DataDragon::new(Some("en_US".to_string())).await;
    match data_dragon {
        Ok(data_dragon) => {
            let data = Data::new(name.clone(), role.clone(), rank, region);
            let fut_winrate = data.winrate();
            let fut_pickrate = data.pick_rate();
            let fut_banrate = data.ban_rate();
            let fut_champion_json = data_dragon.champion_json();
            let (
                winrate,
                pickrate,
                banrate,
                champion_json,
            ) = futures::join!(
                fut_winrate,
                fut_pickrate,
                fut_banrate,
                fut_champion_json,
            );
        
            match winrate {
                Ok(win_rate) => {
                    match pickrate {
                        Ok(pick_rate) => {
                            match banrate {
                                Ok(ban_rate) => {
                                    match champion_json {
                                        Ok(json) => {
                                            let id = &json.data.get(&name).unwrap().id;
                                            let url = format!(
                                                "https://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
                                                &data_dragon.version,
                                                &id
                                            );
                                            let local_image = format!("/{0}/{0}.png", id);
                                            Ok(ChampionInfo {
                                                url,
                                                local_image,
                                                win_rate,
                                                pick_rate,
                                                ban_rate,
                                            })
                                        }
                                        Err(err) => Err(err),
                                    }
                                }
                                Err(err) => Err(err),
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    }
}