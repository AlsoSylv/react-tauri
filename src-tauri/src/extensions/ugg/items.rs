use serde_json::Value;

use crate::{core::data_dragon, errors};

use data_dragon::DataDragon;
use errors::ErrorMap;

use super::{constants, structs};

use constants::DATA;
use structs::{ItemValues, ItemsMap};

impl super::Data {
    /// Returns items from the UGG API these can be empty
    pub async fn items(&self, request: Result<Value, ErrorMap>) -> Result<ItemsMap, ErrorMap> {
        let data_dragon = DataDragon::new(Some(&self.lang)).await;
        let mut items_map = ItemsMap::new();
        let items_array = items_map.as_array_mut();

        match data_dragon {
            Ok(data_dragon) => {
                let items = data_dragon.item_json().await;

                match request {
                    Ok(json) => {
                        match items {
                            Ok(items) => {
                                if let Some(map) = items["data"].as_object() {
                                    for (key, item_data) in map {
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
                                            &data_dragon.version, &image
                                        );
                                        // TODO: We can get the specific win rates of each of these sets rather easily
                                        let ugg_maps = [
                                            &json[DATA["starting_items"]][2],
                                            &json[DATA["mythic_and_core"]][2],
                                            &json[DATA["other_items"]][0],
                                            &json[DATA["other_items"]][1],
                                            &json[DATA["other_items"]][2],
                                        ];

                                        for n in 0..5 {
                                            if let Some(current_map) = ugg_maps[n].as_array() {
                                                current_map.iter().for_each(|y| {
                                                    if y.is_array()
                                                        && (&y[0].to_string() == key
                                                            || &y.to_string() == key)
                                                    {
                                                        items_array[n].push(ItemValues::new(
                                                            name,
                                                            cost,
                                                            description,
                                                            image,
                                                            &url,
                                                        ))
                                                    }
                                                })
                                            };
                                        }
                                    }

                                    Ok(items_map)
                                } else {
                                    unreachable!()
                                }
                            }
                            Err(err) => Err(ErrorMap::DataDragonErrors(err)),
                        }
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(ErrorMap::DataDragonErrors(err)),
        }
    }
}
