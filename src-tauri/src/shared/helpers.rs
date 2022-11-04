use cached::proc_macro::cached;
use serde_json::{json, Value};

use crate::{Active, PrimaryTree, RuneImages, SecondaryTree};

use super::data_dragon::{self};

#[cached(size = 25, result = true)]
pub async fn champion_id(name: String) -> Result<i64, i64> {
    let champion_name = format!("{}", name.clone());

    let request = data_dragon::champion_json().await;
    match request {
        Ok(json) => Ok(json.data[&champion_name].key.parse().unwrap()),
        Err(err) => Err(err),
    }
}

pub async fn create_rune_page(
    name: String,
    primary_id: String,
    secondary_id: String,
    selected_perks: [i64; 9],
) -> Value {
    let rune_page = json!({
        "name": name,
        "primaryStyleId": primary_id,
        "subStyleId": secondary_id,
        "selectedPerkIds": selected_perks
    });
    return rune_page;
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChampionNames {
    pub label: String,
    pub value: String,
    pub url: String,
}

#[cached]
pub async fn all_champion_names() -> Result<Vec<ChampionNames>, i64> {
    let mut champions = Vec::new();
    let fut_champ_json = data_dragon::champion_json();
    let fut_version = self::data_dragon::data_dragon_version();
    let (champ_json, version) = futures::join!(fut_champ_json, fut_version);
    match champ_json {
        Ok(json) => {
            match version {
                Ok(version) => {
                    for (champ_key, champ) in json.data.iter() {
                        let key = &champ.id;
                        champions.push(ChampionNames {
                          label: champ.clone().name,
                          value: champ_key.to_string(),
                          url: format!("https://ddragon.leagueoflegends.com/cdn/{version}/img/champion/{key}.png")
                        });
                    }
                    Ok(champions)
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub async fn all_rune_images(tree_id_one: i64, tree_id_two: i64) -> Result<RuneImages, i64> {
    let request = data_dragon::runes_json().await;
    let mut tree_one_names: PrimaryTree = PrimaryTree {
        slot_one: Vec::new(),
        slot_two: Vec::new(),
        slot_three: Vec::new(),
        slot_four: Vec::new(),
    };
    let mut tree_two_names: SecondaryTree = SecondaryTree {
        slot_one: Vec::new(),
        slot_two: Vec::new(),
        slot_three: Vec::new(),
    };
    match request {
        Ok(json) => {
            for rune in json.iter() {
                if &rune.id == &tree_id_one {
                    for (position, slots) in rune.slots.iter().enumerate() {
                        for runes in &slots.runes {
                            match position {
                                0 => tree_one_names.slot_one.push(Active {
                                    name: runes.name.clone(),
                                    image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                        .to_string()
                                        + &runes.icon.clone(),
                                    active: false,
                                    id: runes.id,
                                    local_image: format!("/{0}/{1}.png", rune.key, runes.key)
                                }),
                                1 => tree_one_names.slot_two.push(Active {
                                    name: runes.name.clone(),
                                    image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                        .to_string()
                                        + &runes.icon.clone(),
                                    active: false,
                                    id: runes.id,
                                    local_image: format!("/{0}/{1}.png", rune.key, runes.key)
                                }),
                                2 => tree_one_names.slot_three.push(Active {
                                    name: runes.name.clone(),
                                    image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                        .to_string()
                                        + &runes.icon.clone(),
                                    active: false,
                                    id: runes.id,
                                    local_image: format!("/{0}/{1}.png", rune.key, runes.key)
                                }),
                                3 => tree_one_names.slot_four.push(Active {
                                    name: runes.name.clone(),
                                    image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                        .to_string()
                                        + &runes.icon.clone(),
                                    active: false,
                                    id: runes.id,
                                    local_image: format!("/{0}/{1}.png", rune.key, runes.key)
                                }),
                                _ => unreachable!(),
                            }
                        }
                    }
                } else if &rune.id == &tree_id_two {
                    for i in 1..4 {
                        let slot = &rune.slots[i];
                        for runes in &slot.runes {
                            match i {
                                1 => tree_two_names.slot_one.push(Active {
                                    name: runes.name.clone(),
                                    image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                        .to_string()
                                        + &runes.icon.clone(),
                                    active: false,
                                    id: runes.id,
                                    local_image: format!("/{0}/{1}.png", rune.key, runes.key)
                                }),
                                2 => tree_two_names.slot_two.push(Active {
                                    name: runes.name.clone(),
                                    image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                        .to_string()
                                        + &runes.icon.clone(),
                                    active: false,
                                    id: runes.id,
                                    local_image: format!("/{0}/{1}.png", rune.key, runes.key)
                                }),
                                3 => tree_two_names.slot_three.push(Active {
                                    name: runes.name.clone(),
                                    image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                        .to_string()
                                        + &runes.icon.clone(),
                                    active: false,
                                    id: runes.id,
                                    local_image: format!("/{0}/{1}.png", rune.key, runes.key)
                                }),
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
            let rune_names = RuneImages {
                primary_runes: tree_one_names,
                secondary_runes: tree_two_names,
            };
            Ok(rune_names)
        }
        Err(err) => Err(err),
    }
}
