use crate::{core::helpers, errors, frontend_types};

use errors::{ErrorMap, UGGDataError};
use frontend_types::RuneImages;

use super::structs::Overview;

impl super::Data {
    /// Returns runes from the UGG API
    /// this heavily uses mutability to
    /// avoid duplication of variables
    pub async fn rune_tuple(
        &self,
        request: Result<Overview, ErrorMap>,
    ) -> Result<(RuneImages, [i64; 2], Vec<i64>), ErrorMap> {
        match request {
            Ok(json) => {
                let json = &json.perks;
                let rune_ids = &json[4];
                let Some(tree_id_one) = &json[2].as_i64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let Some(tree_id_two) = &json[3].as_i64() else {
                    return Err(ErrorMap::UGGError(UGGDataError::MatchesError));
                };

                let all_runes =
                    helpers::runes::all_rune_images(*tree_id_one, *tree_id_two, &self.lang).await;
                match all_runes {
                    Ok(mut all_runes) => {
                        let mut used_rune_ids = Vec::new();
                        let mut slots = all_runes.as_array_mut();

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
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }
}
