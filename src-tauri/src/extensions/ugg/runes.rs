use crate::{frontend_types, core::helpers, errors};

use frontend_types::{Active, RuneImages};
use errors::{ErrorMap, UGGDataError};
use ErrorMap::DataDragonErrors;
use serde_json::Value;

use super::{structs, constants};

impl structs::Data {
    pub async fn rune_tuple(&self, request: Result<Value, ErrorMap>) -> Result<(RuneImages, [i64; 2], Vec<i64>), ErrorMap>{
        match request {
            Ok(json) => {
                let json = &json[constants::DATA["perks"]];
                let rune_ids = &json[4];
                let Some(tree_id_one) = &json[2].as_i64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(tree_id_two) = &json[3].as_i64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let all_runes = helpers::runes::all_rune_images(*tree_id_one, *tree_id_two, &self.lang).await;
                match all_runes {
                    Ok(mut all_runes) => {
                        let mut used_rune_ids = Vec::new();
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
                                current_slot.iter_mut().for_each(|i| {
                                    if rune_ids[n] == i.id {
                                        i.active = true;
                                        used_rune_ids.push(i.id);
                                    }
                                });
                            });
                        }
                        Ok((all_runes, [*tree_id_one, *tree_id_two], used_rune_ids))
                    }
                    Err(err) => Err(DataDragonErrors(err)),
                }
            }
            Err(err) => Err(err),
        }    
    }
}
