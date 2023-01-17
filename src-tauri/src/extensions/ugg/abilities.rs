use serde_json::Value;

use crate::{
    core::{community_dragon::CommunityDragon, data_dragon},
    errors,
    frontend_types::{AbilitiesMap, AbilitiesValue, Passive},
};

use data_dragon::DataDragon;
use errors::{DataDragonError, ErrorMap, UGGDataError};

use ErrorMap::{DataDragonErrors, UGGError};

use super::structs::Overview;

impl super::Data {
    /// Returns abilities from the UGG API
    pub async fn abilities(
        &self,
        request: Result<Overview, ErrorMap>,
    ) -> Result<AbilitiesMap, ErrorMap> {
        match request {
            Ok(json) => {
                let Some(abilities_order) = json.abilities[2].as_array() else {
                    return Err(UGGError(UGGDataError::NoAbilityOrder))
                };

                if let Ok(data_dragon) = DataDragon::new(Some(&self.lang)).await {
                    if let Ok(json) = data_dragon.champ_full(self.name.value.key.clone()).await {
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
                                    &data_dragon.version, &passive
                                ),
                            ),

                            q: AbilitiesValue::new(
                                "Q",
                                q_image,
                                format!(
                                    "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                    &data_dragon.version, &q_image
                                ),
                            ),

                            w: AbilitiesValue::new(
                                "W",
                                w_image,
                                format!(
                                    "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                    &data_dragon.version, &w_image
                                ),
                            ),

                            e: AbilitiesValue::new(
                                "E",
                                e_image,
                                format!(
                                    "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                    &data_dragon.version, &e_image
                                ),
                            ),

                            r: AbilitiesValue::new(
                                "R",
                                r_image,
                                format!(
                                    "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                    &data_dragon.version, &r_image
                                ),
                            ),
                        };

                        split_abilities(&mut abilities.as_array_mut(), abilities_order);
                        Ok(abilities)
                    } else {
                        community_dragon_abilities(&self.lang, self.name.value.id, abilities_order)
                            .await
                    }
                } else {
                    community_dragon_abilities(&self.lang, self.name.value.id, abilities_order)
                        .await
                }
            }
            Err(err) => Err(err),
        }
    }
}

async fn community_dragon_abilities(
    lang: &str,
    id: i64,
    abilities_order: &[Value],
) -> Result<AbilitiesMap, ErrorMap> {
    let community_dragon = CommunityDragon::new(lang);
    let champ_json = community_dragon.champs_full(id).await;
    match champ_json {
        Ok(json) => {
            let spells = json.spells;
            let passive = Passive::new_cd(format!(
                "https://cdn.communitydragon.org/latest/champion/{}/ability-icon/p",
                id
            ));
            let q = AbilitiesValue::new_cd(
                &spells[0].name.to_uppercase(),
                format!(
                    "https://cdn.communitydragon.org/latest/champion/{}/ability-icon/q",
                    id
                ),
            );

            let w = AbilitiesValue::new_cd(
                &spells[1].name.to_uppercase(),
                format!(
                    "https://cdn.communitydragon.org/latest/champion/{}/ability-icon/w",
                    id
                ),
            );

            let e = AbilitiesValue::new_cd(
                &spells[2].name.to_uppercase(),
                format!(
                    "https://cdn.communitydragon.org/latest/champion/{}/ability-icon/e",
                    id
                ),
            );

            let r = AbilitiesValue::new_cd(
                &spells[3].name.to_uppercase(),
                format!(
                    "https://cdn.communitydragon.org/latest/champion/{}/ability-icon/r",
                    id
                ),
            );

            let mut map = AbilitiesMap {
                passive,
                q,
                w,
                e,
                r,
            };

            split_abilities(&mut map.as_array_mut(), abilities_order);
            Ok(map)
        }
        Err(err) => Err(ErrorMap::CommunityDragonErrors(err)),
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
