use crate::{frontend_types, plugins, shared};
use plugins::{ugg, lcu};

use frontend_types::ChampionInfo;
use shared::data_dragon::structs::DataDragon; 
use shared::helpers::runes::create_rune_page;

use ugg::structs::Data;
use lcu::runes::push_runes_to_client;

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

pub async fn push_runes(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<i64, i64> {
    let data = Data::new(name.clone(), role.clone(), rank, region);
    let winrate = data.winrate().await;
    let rune_match = data.rune_tuple().await;
    // let (winrate, rune_match) = futures::join!(fut_winrate, fut_rune_match);

    match rune_match {
        Ok((_, tree_ids, rune_ids)) => {
            match winrate {
                Ok(win_rate) => {
                    let page = create_rune_page(format!("{0} {1} {2}", name, role, win_rate), tree_ids[0], tree_ids[1], rune_ids).await;
                    let result = push_runes_to_client(page).await;
                    match result {
                        Ok(ok) => Ok(ok),
                        Err(err) => Err(err)
                    }
                }
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}