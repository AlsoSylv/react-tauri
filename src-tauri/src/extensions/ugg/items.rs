use crate::{core::data_dragon::structs::DataDragon, errors::ErrorMap};

use super::{structs::{self, ItemsMap, ItemValues}, json::overview, constants::DATA};

impl structs::Data {
    
    pub async fn items(&self) -> Result<ItemsMap, ErrorMap> {
        let data_dragon = DataDragon::new(Some(&self.lang)).await;
        let mut items_map = 
        ItemsMap { 
            start: Vec::new(), 
            core: Vec::new(), 
            fourth: Vec::new(), 
            fifth: Vec::new(), 
            sixth: Vec::new() 
        };
        
        match data_dragon {
            Ok(data_dragon) => {
                let fut_request = overview(
                    &self.name.value.id, 
                    &self.role, 
                    &self.rank, 
                    &self.region,
                    &self.lang,
                );
                let fut_items = data_dragon.item_json();
        
                let (
                    request, 
                    items
                ) = futures::join!(
                    fut_request, 
                    fut_items
                );
                match request {
                    Ok(json) => {
                        match items {
                            Ok(items) => {
                                let start = json[DATA["starting_items"]][2].as_array();
                                let mythic = json[DATA["mythic_and_core"]][2].as_array();
                                let fourth = json[DATA["other_items"]][0].as_array();
                                let fifth = json[DATA["other_items"]][1].as_array();
                                let sixth = json[DATA["other_items"]][2].as_array();
                                    for (key, item_data) in items["data"].as_object().unwrap()  {
                                        match start {
                                            Some(start) => {
                                                for i in start {
                                                    if &i.to_string() == key {
                                                        let image = item_data["image"]["full"].as_str().unwrap().to_string();
                                                        items_map.start.push(
                                                            ItemValues { 
                                                            name: item_data["name"].as_str().unwrap().to_string(), 
                                                            cost: item_data["gold"]["base"].as_i64().unwrap().to_string(), 
                                                            description: item_data["description"].as_str().unwrap().to_string(), 
                                                            local_image: image.clone(), 
                                                            url: format!(
                                                                "http://ddragon.leagueoflegends.com/cdn/{}/img/item/{}",
                                                                &data_dragon.version,
                                                                &image
                                                            ),
                                                        })
                                                    }
                                                }
                                            },
                                            None => (),
                                        }
                                        match mythic {
                                            Some(mythic) => {
                                                for i in mythic {
                                                    if &i.to_string() == key {
                                                        let image = item_data["image"]["full"].as_str().unwrap().to_string();
                                                        items_map.core.push(
                                                            ItemValues { 
                                                            name: item_data["name"].as_str().unwrap().to_string(), 
                                                            cost: item_data["gold"]["base"].as_i64().unwrap().to_string(), 
                                                            description: item_data["description"].as_str().unwrap().to_string(), 
                                                            local_image: image.clone(), 
                                                            url: format!(
                                                                "http://ddragon.leagueoflegends.com/cdn/{}/img/item/{}",
                                                                &data_dragon.version,
                                                                &image
                                                            ),
                                                        })
                                                    }
                                                }
                                            },
                                            None => (),
                                        }
                                        match fourth {
                                            Some(fouth) => {
                                                for y in fouth {
                                                    if y.is_array() {
                                                        if &y[0].to_string() == key {
                                                            let image = item_data["image"]["full"].as_str().unwrap().to_string();
                                                            items_map.fourth.push(
                                                                ItemValues { 
                                                                name: item_data["name"].as_str().unwrap().to_string(), 
                                                                cost: item_data["gold"]["base"].as_i64().unwrap().to_string(), 
                                                                description: item_data["description"].as_str().unwrap().to_string(), 
                                                                local_image: image.clone(), 
                                                                url: format!(
                                                                    "http://ddragon.leagueoflegends.com/cdn/{}/img/item/{}",
                                                                    &data_dragon.version,
                                                                    &image
                                                                ),
                                                            })
                                                        }
                                                    } else {
                                                        break;
                                                    }
                                                }
                                            },
                                            None => (),
                                        }
                                        match fifth {
                                            Some(fifth) => {
                                                for y in fifth {
                                                    if y.is_array() {
                                                        if &y[0].to_string() == key {
                                                            let image = item_data["image"]["full"].as_str().unwrap().to_string();
                                                            items_map.fifth.push(
                                                                ItemValues { 
                                                                name: item_data["name"].as_str().unwrap().to_string(), 
                                                                cost: item_data["gold"]["base"].as_i64().unwrap().to_string(), 
                                                                description: item_data["description"].as_str().unwrap().to_string(), 
                                                                local_image: image.clone(), 
                                                                url: format!(
                                                                    "http://ddragon.leagueoflegends.com/cdn/{}/img/item/{}",
                                                                    &data_dragon.version,
                                                                    &image
                                                                ),
                                                            })
                                                        }
                                                    } else {
                                                        break;
                                                    }
                                                }
                                            },
                                            None => (),
                                        }
                                        match sixth {
                                            Some(sixth) => {
                                                for y in sixth {
                                                    if y.is_array() {
                                                        if &y[0].to_string() == key {
                                                            let image = item_data["image"]["full"].as_str().unwrap().to_string();
                                                            items_map.sixth.push(
                                                                ItemValues { 
                                                                name: item_data["name"].as_str().unwrap().to_string(), 
                                                                cost: item_data["gold"]["base"].as_i64().unwrap().to_string(), 
                                                                description: item_data["description"].as_str().unwrap().to_string(), 
                                                                local_image: image.clone(), 
                                                                url: format!(
                                                                    "http://ddragon.leagueoflegends.com/cdn/{}/img/item/{}",
                                                                    &data_dragon.version,
                                                                    &image
                                                                ),
                                                            })
                                                        }
                                                    } else {
                                                        break;
                                                    }
                                                }
                                            },
                                            None => (),
                                        }
                                    }
                                Ok(items_map)
                            },
                            Err(err) => Err(ErrorMap::DataDragonErrors(err))
                        }
                    },
                    Err(err) => Err(err),
                }
            },
            Err(err) => Err(ErrorMap::DataDragonErrors(err)),
        }
    }
}
