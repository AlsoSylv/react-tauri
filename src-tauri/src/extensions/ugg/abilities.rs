use serde_json::Value;

use crate::{core::data_dragon, errors};

use errors::{ErrorMap, UGGDataError, DataDragonError};
use data_dragon::structs::DataDragon;

use ErrorMap::{UGGError, DataDragonErrors};

use super::{structs, constants};

use structs::{AbilitiesMap, AbilitiesValue, Passive};
use constants::DATA;

impl structs::Data {
    pub async fn abilities(&self, request: Result<Value, ErrorMap>) -> Result<AbilitiesMap, ErrorMap> {
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
                                        q_image, 
                                        format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &q_image
                                        ) 
                                    ), 
        
                                    w: AbilitiesValue::new(
                                        w_image, 
                                        format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &w_image
                                        ) 
                                    ),
        
                                    e: AbilitiesValue::new(
                                        e_image, 
                                        format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &e_image
                                        ) 
                                    ),
        
                                    r: AbilitiesValue::new(
                                        r_image, 
                                        format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &r_image
                                        ) 
                                    ),
                                };
                                
                                split_abilities(abilities.as_array_mut(), abilities_order);
                                Ok(abilities)
                            },
                            Err(err) => Err(DataDragonErrors(err)),
                        }
                    },
                    Err(err) => Err(err),
                }
            },
            Err(err) => Err(DataDragonErrors(err)), 
        }
    }
}

fn split_abilities(maps: [&mut AbilitiesValue; 4], abilities: &Vec<Value>) {
    abilities.iter().for_each(|y| {
        if let Some(y) = y.as_str() {
            let x = String::from(y);
            match y {
                "Q" => {
                    maps[0].order.push(x);
                    maps[1].order.push("".to_string());
                    maps[2].order.push("".to_string());
                    maps[3].order.push("".to_string());
                },
                "W" => {
                    maps[0].order.push("".to_string());
                    maps[1].order.push(x);
                    maps[2].order.push("".to_string());
                    maps[3].order.push("".to_string());
                },
                "E" => {
                    maps[0].order.push("".to_string());
                    maps[1].order.push("".to_string());
                    maps[2].order.push(x);
                    maps[3].order.push("".to_string());
                },
                "R" => {
                    maps[0].order.push("".to_string());
                    maps[1].order.push("".to_string());
                    maps[2].order.push("".to_string());
                    maps[3].order.push(x)
                },
                _ => ()
            }
        }
    })
}