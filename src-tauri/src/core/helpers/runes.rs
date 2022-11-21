use crate::{frontend_types, core::data_dragon::structs::DataDragon, errors::DataDragonError};

use frontend_types::{PrimaryTree, SecondaryTree, Active, RuneImages}; 

pub async fn all_rune_images(tree_id_one: i64, tree_id_two: i64, language: &str) -> Result<RuneImages, DataDragonError> {
    let data_dragon = DataDragon::new(Some(language)).await;
    match data_dragon {
        Ok(data_dragon) => {
            let request = data_dragon.runes_json().await;
            let mut tree_one_names = 
            PrimaryTree {
                slot_one: Vec::new(),
                slot_two: Vec::new(),
                slot_three: Vec::new(),
                slot_four: Vec::new(),
            };
            let mut tree_two_names = 
            SecondaryTree {
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
                                        0 => tree_one_names.slot_one.push(
                                            Active::new(
                                            &runes.name,
                                            format!(
                                                "http://ddragon.leagueoflegends.com/cdn/img/{}",
                                                &runes.icon
                                            ),
                                            runes.id,
                                            format!("/{0}/{1}.png", rune.key, runes.key),
                                            &runes.long_desc
                                        )),
                                        1 => tree_one_names.slot_two.push(
                                            Active::new(
                                                &runes.name,
                                                format!(
                                                    "http://ddragon.leagueoflegends.com/cdn/img/{}",
                                                    &runes.icon
                                                ),
                                                runes.id,
                                                format!("/{0}/{1}.png", rune.key, runes.key),
                                                &runes.long_desc
                                            )),
                                        2 => tree_one_names.slot_three.push(
                                            Active::new(
                                                &runes.name,
                                                format!(
                                                    "http://ddragon.leagueoflegends.com/cdn/img/{}",
                                                    &runes.icon
                                                ),
                                                runes.id,
                                                format!("/{0}/{1}.png", rune.key, runes.key),
                                                &runes.long_desc
                                            )),
                                        3 => tree_one_names.slot_four.push(
                                            Active::new(
                                                &runes.name,
                                                format!(
                                                    "http://ddragon.leagueoflegends.com/cdn/img/{}",
                                                    &runes.icon
                                                ),
                                                runes.id,
                                                format!("/{0}/{1}.png", rune.key, runes.key),
                                                &runes.long_desc
                                            )),
                                        _ => unreachable!(),
                                    }
                                }
                            }
                        } else if &rune.id == &tree_id_two {
                            for i in 1..4 {
                                let slot = &rune.slots[i];
                                for runes in &slot.runes {
                                    match i {
                                        1 => tree_two_names.slot_one.push(
                                            Active::new(
                                                &runes.name,
                                                format!(
                                                    "http://ddragon.leagueoflegends.com/cdn/img/{}",
                                                    &runes.icon
                                                ),
                                                runes.id,
                                                format!("/{0}/{1}.png", rune.key, runes.key),
                                                &runes.long_desc
                                            )),
                                        2 => tree_two_names.slot_two.push(
                                            Active::new(
                                                &runes.name,
                                                format!(
                                                    "http://ddragon.leagueoflegends.com/cdn/img/{}",
                                                    &runes.icon
                                                ),
                                                runes.id,
                                                format!("/{0}/{1}.png", rune.key, runes.key),
                                                &runes.long_desc
                                            )),
                                        3 => tree_two_names.slot_three.push(
                                            Active::new(
                                                &runes.name,
                                                format!(
                                                    "http://ddragon.leagueoflegends.com/cdn/img/{}",
                                                    &runes.icon
                                                ),
                                                runes.id,
                                                format!("/{0}/{1}.png", rune.key, runes.key),
                                                &runes.long_desc
                                            )),
                                        _ => unreachable!(),
                                    }
                                }
                            }
                        }
                    }
                    Ok(
                        RuneImages {
                            primary_runes: tree_one_names,
                            secondary_runes: tree_two_names,
                        }
                    )
                }
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    }
}
