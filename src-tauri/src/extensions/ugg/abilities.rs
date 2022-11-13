use crate::core::data_dragon::structs::DataDragon;

use super::{structs::{self, AbilitiesMap, AbilitiesValue, Passive}, json::overview, constants::DATA};

impl structs::Data {
    pub async fn abilities(&self) -> Result<AbilitiesMap, i64> {
        let data_dragon = DataDragon::new(Some("en_US")).await;
        match data_dragon {
            Ok(data_dragon) => {
                let fut_abilities = overview(
                    self.name.clone(), 
                    self.role.clone(), 
                    self.rank.clone(), 
                    self.region.clone()
                );
                let fut_champ_json = data_dragon.champ_full(self.name.clone());
                let (
                    abilities, 
                    champ_json, 
                ) = futures::join!(
                        fut_abilities, 
                        fut_champ_json, 
                    );
                
                match abilities {
                    Ok(json) => {
                        let Some(abilities_order) = json[DATA["abilities"]][2].as_array() else {
                            return Err(207)
                        };
                        match champ_json {
                            Ok(json) => {
                                let champ_json = json.data[&self.name].clone();
        
                                let possible_passive = &champ_json["passive"]["image"]["full"];
        
                                let spells = &champ_json["spells"];
        
                                let Some(passive) = possible_passive.as_str() else {
                                    return Err(104);
                                };
                                
                                let Some(q_image) = spells[0]["image"]["full"].as_str() else {
                                    return Err(104);
                                };
        
                                let Some(w_image) = spells[1]["image"]["full"].as_str() else {
                                    return Err(104);
                                };
        
                                let Some(e_image) = spells[2]["image"]["full"].as_str() else {
                                    return Err(104);
                                };
        
                                let Some(r_image) = spells[3]["image"]["full"].as_str() else {
                                    return Err(104);
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
        
                                for y in abilities_order {
                                    if y.is_string() {
                                        match y.as_str().unwrap() {
                                            "Q" => {
                                                abilities.q.order.push(y.as_str().unwrap().to_string());
                                                abilities.w.order.push("".to_string());
                                                abilities.e.order.push("".to_string());
                                                abilities.r.order.push("".to_string());
                                            },
                                            "W" => {
                                                abilities.q.order.push("".to_string());
                                                abilities.w.order.push(y.as_str().unwrap().to_string());
                                                abilities.e.order.push("".to_string());
                                                abilities.r.order.push("".to_string());
                                            },
                                            "E" => {
                                                abilities.q.order.push("".to_string());
                                                abilities.w.order.push("".to_string());
                                                abilities.e.order.push(y.as_str().unwrap().to_string());
                                                abilities.r.order.push("".to_string());
                                            },
                                            "R" => {
                                                abilities.q.order.push("".to_string());
                                                abilities.w.order.push("".to_string());
                                                abilities.e.order.push("".to_string());
                                                abilities.r.order.push(y.as_str().unwrap().to_string())
                                            },
                                            _ => break
                                        }
                                    } else {
                                        break
                                    }
                                }
                                Ok(abilities)
                            },
                            Err(err) => Err(err),
                        }
                    },
                    Err(err) => Err(err),
                }
            },
            Err(err) => Err(err), 
        }
    }
}