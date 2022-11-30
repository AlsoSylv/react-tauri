use serde_json::Value;

use crate::{core::data_dragon, errors};

use data_dragon::DataDragon;
use errors::{DataDragonError, ErrorMap, UGGDataError};

use ErrorMap::{DataDragonErrors, UGGError};

use super::{constants, structs};

use constants::DATA;
use structs::{AbilitiesMap, AbilitiesValue, Passive};

impl structs::Data {
    /// Returns abilities from the UGG API
    pub async fn abilities(
        &self,
        request: Result<Value, ErrorMap>,
    ) -> Result<AbilitiesMap, ErrorMap> {
        let data_dragon = DataDragon::new(Some(&self.lang)).await;
        match data_dragon {
            Ok(data_dragon) => {
                let champ_json = data_dragon.champ_full(self.name.value.key.clone()).await;
                match request {
                    Ok(json) => {
                        let Some(abilities_order) = json[DATA["abilities"]][2].as_array() else {
                            return Err(UGGError(UGGDataError::NoAbilityOrder))
                        };
                        match champ_json {
                            Ok(json) => {
                                let champ_json = json.data[&self.name.value.key].clone();

                                let possible_passive = &champ_json["passive"]["image"]["full"];

                                let spells = &champ_json["spells"];

                                let Some(passive) = possible_passive.as_str() else {
                                    return Err(DataDragonErrors(DataDragonError::DataDragonMissing));
                                };

                                let Some(q_image) = spells[0]["image"]["full"].as_str() else {
                                    return Err(DataDragonErrors(DataDragonError::DataDragonMissing));
                                };

                                let Some(w_image) = spells[1]["image"]["full"].as_str() else {
                                    return Err(DataDragonErrors(DataDragonError::DataDragonMissing));
                                };

                                let Some(e_image) = spells[2]["image"]["full"].as_str() else {
                                    return Err(DataDragonErrors(DataDragonError::DataDragonMissing));
                                };

                                let Some(r_image) = spells[3]["image"]["full"].as_str() else {
                                    return Err(DataDragonErrors(DataDragonError::DataDragonMissing));
                                };

                                let mut abilities = AbilitiesMap {
                                    passive: Passive::new(
                                        passive,
                                        format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/passive/{}",
                                            &data_dragon.version,
                                            &passive
                                        )
                                    ),

                                    q: AbilitiesValue::new(
                                        "Q",
                                        q_image,
                                        format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &q_image
                                        )
                                    ),

                                    w: AbilitiesValue::new(
                                        "W",
                                        w_image,
                                        format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &w_image
                                        )
                                    ),

                                    e: AbilitiesValue::new(
                                        "E",
                                        e_image,
                                        format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &e_image
                                        )
                                    ),

                                    r: AbilitiesValue::new(
                                        "R",
                                        r_image,
                                        format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &r_image
                                        )
                                    ),
                                };

                                split_abilities(&mut abilities.as_array_mut(), abilities_order);
                                Ok(abilities)
                            }
                            Err(err) => Err(DataDragonErrors(err)),
                        }
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(DataDragonErrors(err)),
        }
    }
}

/// Splits the abilities that U.GG provides into sub arrays so that
/// it's easier for the frontend to handle.
fn split_abilities(maps: &mut [&mut AbilitiesValue; 4], abilities: &[Value]) {
    abilities.iter().for_each(|y| {
        if let Some(y) = y.as_str() {
            maps.iter_mut().for_each(|ability| {
                if ability.name == y {
                    ability.order.push(String::from(y));
                } else {
                    ability.order.push(String::from(""));
                }
            });
        }
    })
}
