use once_cell::sync::Lazy;

use crate::{extensions::ugg::Data, frontend_types::ChampionNames};

static UGGDATA: Lazy<Data> = Lazy::new(|| {
    Data::new(
        ChampionNames::new("", "", 498, None),
        "3".to_owned(),
        "Platinum Plus".to_owned(),
        "World".to_owned(),
        "en_US".to_owned(),
    )
});

#[tokio::test]
async fn champ_basic_test() {
    use crate::core::community_dragon::CommunityDragon;

    let community_dragon = CommunityDragon::new("en_US");
    if let Ok(champ_basic) = community_dragon.champs_basic().await {
        assert!(champ_basic[0].id == -1);
        assert!(champ_basic[1].id == 1);
    } else {
        panic!()
    };
}

#[tokio::test]
async fn champ_full_test() {
    use crate::core::community_dragon::CommunityDragon;

    let community_dragon = CommunityDragon::new("en_US");
    if let Ok(champ_full) = community_dragon.champs_full(498).await {
        assert!(champ_full.name == "Xayah");
        assert!(champ_full.key == "Xayah");
    } else {
        panic!()
    }
}

#[tokio::test]
async fn runes_test() {
    use crate::core::community_dragon::CommunityDragon;

    let community_dragon = CommunityDragon::new("en_US");
    if let Ok(runes) = community_dragon.runes().await {
        runes.iter().for_each(|rune| {
            if rune.id < 5000 {
                panic!()
            }
        })
    } else {
        panic!()
    }
}

#[tokio::test]
async fn runes_style_test() {
    use crate::core::community_dragon::CommunityDragon;

    let community_dragon = CommunityDragon::new("en_US");
    if let Ok(runes) = community_dragon.runes_style().await {
        assert!(runes.schema_version == 2);
        assert!(runes.styles[0].id == 8400);
        assert!(!runes.styles[0].is_advanced);
    } else {
        panic!()
    }
}

#[tokio::test]
async fn sort_test() {
    use crate::core::helpers::runes::community_dragon_all_rune_images;

    if let Ok(mut runes) = community_dragon_all_rune_images(8100, 8300, "en_US").await {
        let mut slots = runes.as_array_mut();
        let mut used = Vec::new();
        let mut counter = 0;
        let rune_ids: [i64; 6] = [8135, 8120, 8126, 8112, 8306, 8321];

        slots.iter_mut().for_each(|current_slot| {
            current_slot.iter_mut().for_each(|i| {
                for n in 0..6 {
                    if i.id == rune_ids[n] {
                        i.active = true;
                        counter = counter + 1;
                        used.push(i.id);
                    }
                }
            });
        });
        assert!(used == vec![8112, 8126, 8120, 8135, 8306, 8321]);
        assert!(counter == 6);
    } else {
        panic!()
    };
}

#[tokio::test]
async fn community_dragon_item_test() {
    use crate::core::community_dragon::CommunityDragon;
    use crate::{frontend_types::ItemValues, frontend_types::ItemsMap};
    let lang = "en_US";

    if let Ok(json) = UGGDATA.overview().await {
        let community_dragon = CommunityDragon::new(lang);
        let items = community_dragon.item_json().await;
        let mut items_map = ItemsMap::new();
        let items_array = items_map.as_array_mut();
        match items {
            Ok(items) => {
                let ugg_start_core = [&json.starting_items.ids, &json.mythic_and_core.ids];

                let ugg_others = [
                    &json.other_items[0],
                    &json.other_items[1],
                    &json.other_items[2],
                ];

                for x in items {
                    let name = &x.name;
                    let cost = x.price_total;
                    let description = &x.description;
                    let image = &format!("{}.png", x.id);
                    for n in 0..2 {
                        let current_map = ugg_start_core[n];
                        current_map.as_ref().unwrap().iter().for_each(|y| {
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
                            if y.unwrap() == x.id
                            {
                                items_array[n].push(ItemValues::new(
                                    name,
                                    cost,
                                    description,
                                    image,
                                    &url(x.icon_path.clone()),
                                ));
                            }
                        })
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
                            }
                        })
                    }
                }
                println!("{:?}", items_map);
            }
            Err(err) => panic!("{:?}", err),
        }
    } else {
        panic!()
    }
}
