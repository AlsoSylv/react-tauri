use crate::{
    core::community_dragon::new_community_dragon,
    errors,
    frontend_types::{ItemValues, ItemsMap, LCUItemsMap, LCUItemsValue},
};

use errors::ErrorMap;

use super::structs::Overview;

impl super::Data<'_> {
    /// Returns items from the UGG API these can be empty
    pub async fn items(
        &self,
        request: &Result<Overview, ErrorMap>,
    ) -> Result<(ItemsMap, LCUItemsMap), ErrorMap> {
        match request {
            Ok(json) => {
                if let Ok(version) = &self.data_dragon.get_version().await {
                    if let Ok(items) = &self.data_dragon.item_json(version).await {
                        let mut items_map = ItemsMap::new();
                        let mut lcu_items_map = LCUItemsMap::new();
                        let lcu_items_array = lcu_items_map.as_array_mut();
                        let items_array = items_map.as_array_mut();
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
                                    &version, &image
                                );
                                // TODO: We can get the specific win rates of each of these sets rather easily
                                let ugg_start_core =
                                    [&json.starting_items.ids, &json.mythic_and_core.ids];

                                let ugg_others = [
                                    &json.other_items[0],
                                    &json.other_items[1],
                                    &json.other_items[2],
                                ];

                                for n in 0..2 {
                                    if let Some(current_map) = ugg_start_core[n] {
                                        current_map.iter().for_each(|y| {
                                            if let Some(y) = y {
                                                if &y.to_string() == key {
                                                    items_array[n].push(ItemValues::new(
                                                        name,
                                                        cost,
                                                        description,
                                                        image,
                                                        &url,
                                                    ));

                                                    lcu_items_array[n]
                                                        .push(LCUItemsValue::new(key));
                                                }
                                            }
                                        })
                                    };
                                }

                                for n in 0..3 {
                                    let current_map = ugg_others[n];
                                    current_map.iter().for_each(|y| {
                                        if let Some(y) = y.id {
                                            if &y.to_string() == key {
                                                items_array[n + 2].push(ItemValues::new(
                                                    name,
                                                    cost,
                                                    description,
                                                    image,
                                                    &url,
                                                ));

                                                lcu_items_array[n].push(LCUItemsValue::new(key));
                                            }
                                        }
                                    })
                                }
                            }

                            Ok((items_map, lcu_items_map))
                        } else {
                            unreachable!()
                        }
                    } else {
                        community_dragon_items(self.lang, json, self.client).await
                    }
                } else {
                    community_dragon_items(self.lang, json, self.client).await
                }
            }
            Err(err) => Err(err.to_owned()),
        }
    }
}

async fn community_dragon_items(
    lang: Option<&str>,
    json: &Overview,
    client: &reqwest::Client,
) -> Result<(ItemsMap, LCUItemsMap), ErrorMap> {
    let community_dragon = new_community_dragon(lang, client);
    let items = community_dragon.item_json().await;
    let mut items_map = ItemsMap::new();
    let items_array = items_map.as_array_mut();
    let mut lcu_items_map = LCUItemsMap::new();
    let lcu_items_array = lcu_items_map.as_array_mut();
    match items {
        Ok(items) => {
            let ugg_start_core = [&json.starting_items.ids, &json.mythic_and_core.ids];

            let ugg_others = [
                &json.other_items[0],
                &json.other_items[1],
                &json.other_items[2],
            ];

            for x in items {
                let id = x.id;
                let name = &x.name;
                let cost = x.price_total;
                let description = &x.description;
                let image = &format!("{}.png", x.id);
                for n in 0..2 {
                    if let Some(current_map) = ugg_start_core[n] {
                        current_map.iter().for_each(|y| {
                            let url = |item_path: String| {
                                let base_url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default";
                                if let Some(item_path_pos) = item_path.find("/ASSETS/") {
                                    let split = item_path.split_at(item_path_pos);
                                    let url = format!("{}{}", base_url, split.1);
                                    url.to_lowercase()
                                } else {
                                    unreachable!();
                                }
                            };
                            if *y == Some(x.id)
                            {
                                items_array[n].push(ItemValues::new(
                                    name,
                                    cost,
                                    description,
                                    image,
                                    &url(x.icon_path.clone()),
                                ));
                                lcu_items_array[n].push(LCUItemsValue::new(&id.to_string()));
                            }
                        })
                    };
                }

                for n in 0..3 {
                    let current_map = ugg_others[n];
                    current_map.iter().for_each(|y| {
                        let url = |item_path: String| {
                            let base_url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default";
                            if let Some(item_path_pos) = item_path.find("/ASSETS/") {
                                let split = item_path.split_at(item_path_pos);
                                let url = format!("{}{}", base_url, split.1);
                                url.to_lowercase()
                            } else {
                                unreachable!();
                            }
                        };
                        if y.id == Some(x.id)
                        {
                            items_array[n + 2].push(ItemValues::new(
                                name,
                                cost,
                                description,
                                image,
                                &url(x.icon_path.clone()),
                            ));
                            lcu_items_array[n].push(LCUItemsValue::new(&id.to_string()));
                        }
                    })
                }
            }
            Ok((items_map, lcu_items_map))
        }
        Err(err) => Err(ErrorMap::CommunityDragonErrors(err)),
    }
}
