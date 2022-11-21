use serde_json::Value;

use crate::{core::data_dragon, errors};

use data_dragon::structs::DataDragon;
use errors::ErrorMap;

use super::{structs, constants};

use constants::DATA;
use structs::{ItemsMap, ItemValues};

impl structs::Data {
    pub async fn items(&self, request: Result<Value, ErrorMap>) -> Result<ItemsMap, ErrorMap> {
        let data_dragon = DataDragon::new(Some(&self.lang)).await;
        let mut items_map = ItemsMap::new();
        
        match data_dragon {
            Ok(data_dragon) => {
                let items = data_dragon.item_json().await;

                match request {
                    Ok(json) => {
                        match items {
                            Ok(items) => {
                                //TODO We can get the specfic winrates of each of these sets rather easily
                                let start = json[DATA["starting_items"]][2].as_array();
                                let mythic = json[DATA["mythic_and_core"]][2].as_array();
                                let fourth = json[DATA["other_items"]][0].as_array();
                                let fifth = json[DATA["other_items"]][1].as_array();
                                let sixth = json[DATA["other_items"]][2].as_array();
                                if let Some(map) = items["data"].as_object() {
                                    for (key, item_data) in map  {
                                        let Some(image) = item_data["image"]["full"].as_str() else {
                                            unreachable!();
                                        };
                                        let Some(name) = item_data["name"].as_str() else {
                                            unreachable!();
                                        };
                                        let Some(cost) = item_data["gold"]["base"].as_i64() else {
                                            unreachable!();
                                        };
                                        let Some(description) = item_data["description"].as_str() else {
                                            unreachable!();
                                        };
                                        let url = format!(
                                            "http://ddragon.leagueoflegends.com/cdn/{}/img/item/{}",
                                            &data_dragon.version,
                                            &image
                                        );
                                        match start {
                                            Some(start) => {
                                                for i in start {
                                                    if &i.to_string() == key {
                                                        items_map.start.push(
                                                            ItemValues::new(
                                                                name, 
                                                                cost, 
                                                                description, 
                                                                image, 
                                                                &url
                                                            )
                                                        )
                                                    }
                                                }
                                            },
                                            None => (),
                                        }
                                        match mythic {
                                            Some(mythic) => {
                                                for i in mythic {
                                                    if &i.to_string() == key {
                                                        items_map.core.push(
                                                            ItemValues::new(
                                                                name, 
                                                                cost, 
                                                                description, 
                                                                image, 
                                                                &url
                                                            )
                                                        )
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
                                                            items_map.fourth.push(
                                                                ItemValues::new(
                                                                    name, 
                                                                    cost, 
                                                                    description, 
                                                                    image, 
                                                                    &url
                                                                )
                                                            )
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
                                                            items_map.fifth.push(
                                                                ItemValues::new(
                                                                    name, 
                                                                    cost, 
                                                                    description, 
                                                                    image, 
                                                                    &url
                                                                )
                                                            )
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
                                                            items_map.sixth.push(
                                                                ItemValues::new(
                                                                    name, 
                                                                    cost, 
                                                                    description, 
                                                                    image, 
                                                                    &url
                                                                )
                                                            )
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
                                } else {
                                    unreachable!()
                                }
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
