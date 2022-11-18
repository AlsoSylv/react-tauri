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
                                    passive: Passive { 
                                        image: passive.to_string(), 
                                        url: format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/passive/{}",
                                            &data_dragon.version,
                                            &passive
                                        ) 
                                    },
        
                                    q: AbilitiesValue { 
                                        image: q_image.to_string(), 
                                        order: Vec::new(), 
                                        url: format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &q_image
                                        ) 
                                    },
        
                                    w: AbilitiesValue { 
                                        image: w_image.to_string(), 
                                        order: Vec::new(),
                                        url: format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &w_image
                                        ) 
                                    },
        
                                    e: AbilitiesValue { 
                                        image: e_image.to_string(), 
                                        order: Vec::new(), 
                                        url: format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &e_image
                                        ) 
                                    },
        
                                    r: AbilitiesValue { 
                                        image: r_image.to_string(), 
                                        order: Vec::new(), 
                                        url: format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}",
                                            &data_dragon.version,
                                            &r_image
                                        ) 
                                    }, 
                                };
                                let maps: [&mut Vec<String>; 4] = [
                                    &mut abilities.q.order,
                                    &mut abilities.w.order,
                                    &mut abilities.e.order,
                                    &mut abilities.r.order
                                    ];

                                    split_abilities(maps, abilities_order);
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

fn split_abilities(maps: [&mut Vec<String>; 4], abilities: &Vec<Value>) {
    for y in abilities {
        if y.is_string() {
            match y.as_str().unwrap() {
                "Q" => {
                    maps[0].push(y.as_str().unwrap().to_string());
                    maps[1].push("".to_string());
                    maps[2].push("".to_string());
                    maps[3].push("".to_string());
                },
                "W" => {
                    maps[0].push("".to_string());
                    maps[1].push(y.as_str().unwrap().to_string());
                    maps[2].push("".to_string());
                    maps[3].push("".to_string());
                },
                "E" => {
                    maps[0].push("".to_string());
                    maps[1].push("".to_string());
                    maps[2].push(y.as_str().unwrap().to_string());
                    maps[3].push("".to_string());
                },
                "R" => {
                    maps[0].push("".to_string());
                    maps[1].push("".to_string());
                    maps[2].push("".to_string());
                    maps[3].push(y.as_str().unwrap().to_string())
                },
                _ => break
            }
        } else {
            break
        }
    }
}