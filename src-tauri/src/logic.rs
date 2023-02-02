use crate::core::helpers::champs::get_champ_names;
use crate::core::lcu;
use crate::core::lcu::items::push_items_to_client;
use crate::extensions::ugg::data_new;
use crate::frontend_types::ChampionNames;
use crate::{extensions, frontend_types, AppState};

use extensions::ugg;
use frontend_types::ChampionInfo;
use serde_json::json;

use lcu::runes::push_runes_to_client;
use tauri::State;
use ugg::Data;

/// Returns the current pick rate, win rate, ban rate, and tier for each champ as requested by the FE
#[tauri::command]
pub async fn champion_info(
    state: State<'_, AppState>,
    name: ChampionNames,
    role: Option<String>,
    rank: String,
    region: String,
    lang: String,
) -> Result<ChampionInfo, i64> {
    let role: String = match role {
        Some(role) => role,
        None => match Data::no_pos(name.value.id, &state.client).await {
            Ok(role) => role,
            Err(err) => return Err(i64::from(err)),
        },
    };

    let data = data_new(
        name.clone(),
        role,
        rank,
        region,
        Some(&lang),
        &state.data_dragon,
        &state.client,
    );

    let (ranking, overview) = futures::join!(data.ranking(), data.overview());

    let fut_winrate = data.winrate(&overview);
    let fut_pickrate = data.pick_rate(&ranking);
    let fut_banrate = data.ban_rate(&ranking);
    let fut_tier = data.rank(&ranking);
    let fut_role = data.default_pos();

    let fut_runes = data.rune_tuple(&overview);
    let fut_abilities = data.abilities(&overview);
    let fut_shards = data.shard_tuple(&overview);
    let fut_items = data.items(&overview);
    let fut_spells = data.summoners(&overview);

    let (winrate, pickrate, banrate, tier, role, runes, abilities, shards, items, spells) = futures::join!(
        fut_winrate,
        fut_pickrate,
        fut_banrate,
        fut_tier,
        fut_role,
        fut_runes,
        fut_abilities,
        fut_shards,
        fut_items,
        fut_spells
    );

    let local_image = format!("/{0}/{0}.png", &name.value.key);

    let url = match &state.data_dragon.get_version().await {
        Ok(version) => Ok(format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
            &version, &name.value.key
        )),
        Err(err) => Err(err.to_owned() as i64),
    };

    Ok(ChampionInfo {
        url,
        local_image,
        win_rate: winrate.map_err(i64::from),
        pick_rate: pickrate.map_err(i64::from),
        ban_rate: banrate.map_err(i64::from),
        tier: tier.map_err(i64::from),
        role: role.map_err(i64::from),
        runes: match runes {
            Ok((obj, _, _)) => Ok(obj),
            Err(err) => Err(i64::from(err)),
        },
        items: match items {
            Ok((obj, _)) => Ok(obj),
            Err(err) => Err(i64::from(err)),
        },
        abilities: abilities.map_err(i64::from),
        shards: shards.map_err(i64::from),
        spells: spells.map_err(i64::from),
    })
}

/// Manages pushing runes to the client by generating a JSON page and
/// connecting to the LCU API to send runes
#[tauri::command]
pub async fn push_runes(
    state: State<'_, AppState>,
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: String,
) -> Result<i64, i64> {
    let data = data_new(
        name.clone(),
        role.clone(),
        rank,
        region,
        Some(&lang),
        &state.data_dragon,
        &state.client,
    );
    let overview = data.overview().await;
    let fut_winrate = data.winrate(&overview);
    let fut_rune_match = data.rune_tuple(&overview);
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
                Err(_) => Err(data_dragon::DataDragonError::DataDragonMissing as i64),
            }
        }
        Err(_) => Err(data_dragon::DataDragonError::DataDragonMissing as i64),
    }
}

/// Generates a list of all champion names, IDs, Keys, URLs, and a local image
/// that is used by the front end in order to generate a selection list
#[tauri::command]
pub async fn all_champion_names(state: State<'_, AppState>, lang: &str) -> Result<Vec<ChampionNames>, i64> {
    let mut champions = Vec::new();
    match get_champ_names(lang, &mut champions, &state.client, &state.data_dragon).await {
        Ok(()) => Ok(champions),
        Err(err) => Err(i64::from(err)),
    }
}

#[tauri::command]
pub async fn push_items(
    state: State<'_, AppState>,
    name: ChampionNames,
    role: String,
    rank: String,
    region: String,
    lang: String,
) -> Result<i64, i64> {
    let data = data_new(
        name.clone(),
        role.clone(),
        rank,
        region,
        Some(&lang),
        &state.data_dragon,
        &state.client,
    );
    let overview = data.overview().await;
    let fut_winrate = data.winrate(&overview);
    let fut_item_match = data.items(&overview);
    let (winrate, item_match) = futures::join!(fut_winrate, fut_item_match);

    match item_match {
        Ok((_, items)) => {
            let page_name = match winrate {
                Ok(winrate) => {
                    format!("{} build {} WR", name.label, winrate)
                }
                Err(_) => {
                    format!("{} build", name.label)
                }
            };

            let page = json!(
                {
                  "associatedChampions": [
                    name.label
                  ],
                  "blocks": [
                    {
                      "items": items.start,
                      "type": "Starting Items"
                    },
                    {
                        "items": items.core,
                        "type": "Core Items"
                    },
                    {
                        "items": items.fourth,
                        "type": "Fourth"
                    },
                    {
                        "items": items.fifth,
                        "type": "Fifth"
                    },
                    {
                        "items": items.sixth,
                        "type": "Sixth"
                    }
                  ],
                  "title": page_name,
                }
            );

            let result = push_items_to_client(page).await;
            match result {
                Ok(ok) => Ok(ok as i64),
                Err(err) => Err(err as i64),
            }
        }
        Err(err) => Err(i64::from(err)),
    }
}
