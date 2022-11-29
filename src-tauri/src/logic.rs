use crate::core::{data_dragon, lcu};
use crate::errors::DataDragonError;
use crate::extensions::ugg::json::{overview, ranking};
use crate::frontend_types::{ChampionNames, ChampionValue, RunesAndAbilities};
use crate::{extensions, frontend_types};

use data_dragon::structs::DataDragon;
use extensions::ugg;
use frontend_types::ChampionInfo;
use serde_json::json;

use lcu::runes::push_runes_to_client;
use ugg::structs::Data;

//TODO: This shouldn't fail if something goes wrong, it should just send the values that work
/// Returns the current pick rate, win rate, ban rate, and tier for each champ as requested by the FE
#[tauri::command]
pub async fn champion_info(
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: String,
) -> Result<ChampionInfo, i64> {
    let request = ranking(&name.value.id, &role, &rank, &region, &lang).await;
    let data_dragon = DataDragon::new(Some(&lang)).await;

    let data = Data::new(name.clone(), role.clone(), rank, region, lang);
    let fut_winrate = data.winrate(request.clone());
    let fut_pickrate = data.pick_rate(request.clone());
    let fut_banrate = data.ban_rate(request.clone());
    let fut_tier = data.rank(request);

    let (winrate, pickrate, banrate, tier) =
        futures::join!(fut_winrate, fut_pickrate, fut_banrate, fut_tier,);

    match data_dragon {
        Ok(data_dragon) => match winrate {
            Ok(win_rate) => match pickrate {
                Ok(pick_rate) => match banrate {
                    Ok(ban_rate) => {
                        let url = format!(
                            "https://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
                            &data_dragon.version, &name.value.key
                        );
                        let local_image = format!("/{0}/{0}.png", &name.value.key);
                        match tier {
                            Ok(tier) => Ok(ChampionInfo {
                                url,
                                local_image,
                                win_rate,
                                pick_rate,
                                ban_rate,
                                tier,
                            }),
                            Err(err) => Err(i64::from(err)),
                        }
                    }
                    Err(err) => Err(i64::from(err)),
                },
                Err(err) => Err(i64::from(err)),
            },
            Err(err) => Err(i64::from(err)),
        },
        Err(err) => Err(err as i64),
    }
}

/// Manages pushing runes to the client by generating a JSON page and
/// connecting to the LCU API to send runes
#[tauri::command]
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
    let (winrate, rune_match) = futures::join!(fut_winrate, fut_rune_match);

    match rune_match {
        Ok((_, tree_ids, rune_ids)) => match winrate {
            Ok(win_rate) => {
                let page = json!({
                    "name": format!("{0} {1} {2}", name.label, role, win_rate),
                    "primaryStyleId": tree_ids[0],
                    "subStyleId": tree_ids[1],
                    "selectedPerkIds": rune_ids
                });
                let result = push_runes_to_client(page).await;
                match result {
                    Ok(response) => Ok(response as i64),
                    Err(err) => Err(err as i64),
                }
            }
            Err(err) => Err(i64::from(err)),
        },
        Err(err) => Err(i64::from(err)),
    }
}

//TODO: This needs a data dragon fallback, assuming one exists
/// Sends a list of all languages from DataDragon to the front end
#[tauri::command]
pub async fn get_languages() -> Result<Vec<String>, i64> {
    let request = reqwest::get("https://ddragon.leagueoflegends.com/cdn/languages.json").await;
    match request {
        Ok(response) => {
            let langs: Result<Vec<String>, reqwest::Error> = response.json().await;
            match langs {
                Ok(langs) => Ok(langs),
                Err(_) => Err(DataDragonError::DataDragonMissing as i64),
            }
        }
        Err(_) => Err(DataDragonError::DataDragonMissing as i64),
    }
}

// TODO: This shouldn't fail if something goes wrong, it should just send the values that work
/// Sends runes, items, abilities, and stat shards to the front end
/// will later include summoner spells, and in the future will return
/// in the form of a result instead of using pattern matching
#[tauri::command]
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
    let (runes, abilities, shards, items) =
        futures::join!(fut_runes, fut_abilities, fut_shards, fut_items,);

    match runes {
        Ok((runes, _, _)) => match abilities {
            Ok(abilities) => match shards {
                Ok(shards) => match items {
                    Ok(items) => Ok(RunesAndAbilities {
                        runes,
                        items,
                        abilities,
                        shards,
                    }),
                    Err(err) => Err(i64::from(err)),
                },
                Err(err) => Err(i64::from(err)),
            },
            Err(err) => Err(i64::from(err)),
        },
        Err(err) => Err(i64::from(err)),
    }
}

/// Generates a list of all champion names, IDs, Keys, URLs, and a local image
/// that is used by the front end in order to generate a selection list
#[tauri::command]
pub async fn all_champion_names(lang: &str) -> Result<Vec<ChampionNames>, i64> {
    let mut champions = Vec::new();
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
                    Ok(champions)
                }
                Err(err) => Err(err as i64),
            }
        }
        Err(err) => Err(err as i64),
    }
}
