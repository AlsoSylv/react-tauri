use crate::{frontend_types::ChampionInfo, plugins::ugg::structs::Data, shared};

pub async fn champion_info(
    name: String,
    role: String,
    rank: String,
    region: String,
) -> Result<ChampionInfo, i64> {
    let rates = Data {
        name: name.clone(), role, rank, region
    };
    let fut_winrate = Data::winrate(&rates);
    let fut_pickrate = Data::pick_rate(&rates);
    let fut_banrate = Data::ban_rate(&rates);
    let fut_champion_json = shared::data_dragon::champion_json();
    let fut_version = shared::data_dragon::data_dragon_version();
    let (
        winrate,
        pickrate,
        banrate,
        champion_json,
        version
    ) = futures::join!(
        fut_winrate,
        fut_pickrate,
        fut_banrate,
        fut_champion_json,
        fut_version
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
                                    match version {
                                        Ok(version) => {
                                            let url = format!("https://ddragon.leagueoflegends.com/cdn/{version}/img/champion/{id}.png");
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
        }
        Err(err) => Err(err),
    }
}