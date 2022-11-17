use crate::core::helpers::structs::ChampionNames;
use crate::core::lcu;
use crate::errors::DataDragonError;
use crate::extensions::ugg::json::{ranking, overview};
use crate::frontend_types::RunesAndAbilities;
use crate::{frontend_types, extensions};
use extensions::ugg;

use frontend_types::ChampionInfo;
use crate::core::data_dragon::structs::DataDragon; 
use crate::core::helpers::runes::create_rune_page;

use ugg::structs::Data;
use lcu::runes::push_runes_to_client;

pub async fn champion_info(
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: String,
) -> Result<ChampionInfo, i64> {
    let data_dragon = DataDragon::new(Some(&lang)).await;
    let request = ranking(&name.value.id, &role, &rank, &region, "en_US").await;
    match data_dragon {
        Ok(data_dragon) => {
            let data = Data::new(name.clone(), role.clone(), rank, region, lang);
            let fut_winrate = data.winrate(request.clone());
            let fut_pickrate = data.pick_rate(request.clone());
            let fut_banrate = data.ban_rate(request.clone());
            let fut_tier = data.rank(request);
            let fut_champion_json = data_dragon.champion_json();
            let (
                winrate,
                pickrate,
                banrate,
                champion_json,
                tier,
            ) = futures::join!(
                fut_winrate,
                fut_pickrate,
                fut_banrate,
                fut_champion_json,
                fut_tier
            );
        
            match winrate {
                Ok(win_rate) => {
                    match pickrate {
                        Ok(pick_rate) => {
                            match banrate {
                                Ok(ban_rate) => {
                                    match champion_json {
                                        Ok(json) => {
                                            let id = &json.data.get(&name.label).unwrap().id;
                                            let url = format!(
                                                "https://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
                                                &data_dragon.version,
                                                &id
                                            );
                                            let local_image = format!("/{0}/{0}.png", id);
                                            match tier {
                                                Ok(tier) => {
                                                    Ok(ChampionInfo {
                                                        url,
                                                        local_image,
                                                        win_rate,
                                                        pick_rate,
                                                        ban_rate,
                                                        tier,
                                                    })
                                                }
                                                Err(err) => Err(i64::from(err))
                                            }
                                        }
                                        Err(err) => Err(i64::from(err)),
                                    }
                                }
                                Err(err) => Err(i64::from(err)),
                            }
                        }
                        Err(err) => Err(i64::from(err)),
                    }
                }
                Err(err) => Err(i64::from(err)),
            }
        },
        Err(err) => Err(i64::from(err)),
    }
}

pub async fn push_runes(
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: String,
) -> Result<i64, i64> {
    let request = ranking(&name.value.id, &role, &rank, &region, &lang).await;
    let request_2 = overview(&name.value.id, &role, &rank, &region, &lang).await;
    let data = Data::new(name.clone(), role.clone(), rank, region, lang);
    let fut_winrate = data.winrate(request);
    let fut_rune_match = data.rune_tuple(request_2);
    let (
        winrate, 
        rune_match
        ) = futures::join!(
            fut_winrate, 
            fut_rune_match
        );

    match rune_match {
        Ok((_, tree_ids, rune_ids)) => {
            match winrate {
                Ok(win_rate) => {
                    let page = create_rune_page(
                        format!("{0} {1} {2}", name.label, role, win_rate), 
                        tree_ids[0], 
                        tree_ids[1], 
                        rune_ids
                    ).await;
                    let result = push_runes_to_client(page).await;
                    match result {
                        Ok(response) => Ok(i64::from(response)),
                        Err(err) => Err(i64::from(err))
                    }
                }
                Err(err) => Err(i64::from(err))
            }
        },
        Err(err) => Err(i64::from(err))
    }
}

pub async fn languages() -> Result<Vec<String>, i64> {
    let request = reqwest::get("https://ddragon.leagueoflegends.com/cdn/languages.json").await;
    match request {
        Ok(response) => {
            let langs: Result<Vec<String>, reqwest::Error> = response.json().await;
            match langs {
                Ok(langs) => Ok(langs),
                Err(_) => Err(i64::from(DataDragonError::DataDragonMissing)),
            }
        },
        Err(_) => Err(i64::from(DataDragonError::DataDragonMissing)),
    }
}

pub async fn runes_and_abilities(
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: String,
) -> Result<RunesAndAbilities, i64> {
    let request = overview(&name.value.id, &role, &rank, &region, &lang).await;
    let data = Data::new(name, role, rank, region, lang);
    let fut_runes = data.rune_tuple(request.clone());
    let fut_abilities = data.abilities(request.clone());
    let fut_shards = data.shard_tuple(request.clone());
    let fut_items = data.items(request);
    let (runes,
        abilities,
        shards,
        items,
    ) = futures::join!(
        fut_runes,
        fut_abilities,
        fut_shards,
        fut_items,
    );

    match runes {
        Ok((runes, _, _)) => {
            match abilities {
                Ok(abilities) => {
                    match shards {
                        Ok(shards) => {
                            match items {
                                Ok(items) => {
                                    Ok(
                                        RunesAndAbilities { 
                                            runes, 
                                            items, 
                                            abilities, 
                                            shards 
                                        }
                                    )
                                },
                                Err(err) => Err(i64::from(err))
                            }
                        },
                        Err(err) => Err(i64::from(err))
                    }
                },
                Err(err) => Err(i64::from(err))
            }
        },
        Err(err) => Err(i64::from(err)),
    }
}
