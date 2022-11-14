use serde_json::{Value, json};

use crate::{frontend_types, core::data_dragon::structs::DataDragon};

use frontend_types::{PrimaryTree, SecondaryTree, Active, RuneImages};

pub async fn create_rune_page(
    name: String,
    primary_id: i64,
    secondary_id: i64,
    selected_perks: Vec<i64>,
) -> Value {
    let rune_page = json!({
        "name": name,
        "primaryStyleId": primary_id,
        "subStyleId": secondary_id,
        "selectedPerkIds": selected_perks
    });
    return rune_page;
}

pub async fn all_rune_images(tree_id_one: i64, tree_id_two: i64, language: &str) -> Result<RuneImages, i64> {
    let data_dragon = DataDragon::new(Some(language)).await;
    match data_dragon {
        Ok(data_dragon) => {
            let request = data_dragon.runes_json().await;
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
                                            local_image: format!("/{0}/{1}.png", rune.key, runes.key),
                                            description: runes.long_desc.clone(),
                                        }),
                                        1 => tree_one_names.slot_two.push(Active {
                                            name: runes.name.clone(),
                                            image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                                .to_string()
                                                + &runes.icon.clone(),
                                            active: false,
                                            id: runes.id,
                                            local_image: format!("/{0}/{1}.png", rune.key, runes.key),
                                            description: runes.long_desc.clone(),
                                        }),
                                        2 => tree_one_names.slot_three.push(Active {
                                            name: runes.name.clone(),
                                            image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                                .to_string()
                                                + &runes.icon.clone(),
                                            active: false,
                                            id: runes.id,
                                            local_image: format!("/{0}/{1}.png", rune.key, runes.key),
                                            description: runes.long_desc.clone(),
                                        }),
                                        3 => tree_one_names.slot_four.push(Active {
                                            name: runes.name.clone(),
                                            image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                                .to_string()
                                                + &runes.icon.clone(),
                                            active: false,
                                            id: runes.id,
                                            local_image: format!("/{0}/{1}.png", rune.key, runes.key),
                                            description: runes.long_desc.clone(),
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
                                            local_image: format!("/{0}/{1}.png", rune.key, runes.key),
                                            description: runes.long_desc.clone(),
                                        }),
                                        2 => tree_two_names.slot_two.push(Active {
                                            name: runes.name.clone(),
                                            image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                                .to_string()
                                                + &runes.icon.clone(),
                                            active: false,
                                            id: runes.id,
                                            local_image: format!("/{0}/{1}.png", rune.key, runes.key),
                                            description: runes.long_desc.clone(),
                                        }),
                                        3 => tree_two_names.slot_three.push(Active {
                                            name: runes.name.clone(),
                                            image: "http://ddragon.leagueoflegends.com/cdn/img/"
                                                .to_string()
                                                + &runes.icon.clone(),
                                            active: false,
                                            id: runes.id,
                                            local_image: format!("/{0}/{1}.png", rune.key, runes.key),
                                            description: runes.long_desc.clone(),
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
        },
        Err(err) => Err(err),
    }
}
