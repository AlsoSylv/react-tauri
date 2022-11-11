use crate::{Active, RuneImages, shared::helpers};

use super::{structs, constants, requests};

impl structs::Data {
    pub async fn rune_tuple(&self) -> Result<(RuneImages, [i64; 2], Vec<i64>), i64>{
        let request = requests::overview(
            self.name.clone(), 
            self.role.clone(), 
            self.rank.clone(), 
            self.region.clone()
        ).await;
        match request {
            Ok(json) => {
                let json = &json[constants::DATA["perks"]];
                let rune_ids = &json[4];
                let Some(tree_id_one) = &json[2].as_i64() else {
                    return Err(206);
                };

                let Some(tree_id_two) = &json[3].as_i64() else {
                    return Err(206);
                };

                let all_runes = helpers::all_rune_images(*tree_id_one, *tree_id_two).await;
                match all_runes {
                    Ok(immutable_all_runes) => {
                        let mut used_rune_ids = Vec::new();
                        let mut all_runes = immutable_all_runes.clone();
                        let mut slots: [&mut Vec<Active>; 7] = [
                            &mut all_runes.primary_runes.slot_one,
                            &mut all_runes.primary_runes.slot_two,
                            &mut all_runes.primary_runes.slot_three,
                            &mut all_runes.primary_runes.slot_four,
                            &mut all_runes.secondary_runes.slot_one,
                            &mut all_runes.secondary_runes.slot_two,
                            &mut all_runes.secondary_runes.slot_three,
                        ];
                        
                        for n in 0..6 {
                            slots.iter_mut().for_each(|current_slot| {
                                current_slot.clone().iter().enumerate().for_each(|i| {
                                    let pos = i.0;
                                    let rune = i.1;
                                    if rune_ids[n] == rune.id {
                                        current_slot[pos] = Active {
                                            name: rune.name.clone(),
                                            image: rune.image.clone(),
                                            active: true,
                                            id: rune.id,
                                            local_image: rune.local_image.clone()
                                        };
                                        used_rune_ids.push(rune.id);
                                    }
                                });
                            });
                        }
                        Ok((all_runes, [*tree_id_one, *tree_id_two], used_rune_ids))
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }    
    }
}
