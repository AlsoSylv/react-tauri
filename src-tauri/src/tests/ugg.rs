use once_cell::sync::Lazy;

use crate::{extensions::ugg::Data, frontend_types::ChampionNames};

static UGGDATA: Lazy<Data> = Lazy::new(|| {
    Data::new(
        ChampionNames::new("", "", 498, None),
        "ADC".to_owned(),
        "Platinum Plus".to_owned(),
        "World".to_owned(),
        "en_US".to_owned(),
    )
});

#[tokio::test]
async fn ranking_structure_test() {
    if let Ok(json) = UGGDATA.ranking().await {
        println!("{:#?}", json);
        assert!(json.is_array());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn wins_test() {
    use crate::extensions::ugg::constants;
    use constants::STATS;

    if let Ok(json) = UGGDATA.ranking().await {
        assert!(json[STATS["wins"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn matches_test() {
    use crate::extensions::ugg::constants;
    use constants::STATS;

    if let Ok(json) = UGGDATA.ranking().await {
        assert!(json[STATS["matches"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn rank_test() {
    use crate::extensions::ugg::constants;
    use constants::STATS;

    if let Ok(json) = UGGDATA.ranking().await {
        assert!(json[STATS["rank"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn total_rank_test() {
    use crate::extensions::ugg::constants;
    use constants::STATS;

    if let Ok(json) = UGGDATA.ranking().await {
        assert!(json[STATS["total_rank"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn bans_test() {
    use crate::extensions::ugg::constants;
    use constants::STATS;

    if let Ok(json) = UGGDATA.ranking().await {
        assert!(json[STATS["bans"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn total_matches_test() {
    use crate::extensions::ugg::constants;
    use constants::STATS;

    if let Ok(json) = UGGDATA.ranking().await {
        assert!(json[STATS["total_matches"]].is_f64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn real_matches_test() {
    use crate::extensions::ugg::constants;
    use constants::STATS;

    if let Ok(json) = UGGDATA.ranking().await {
        assert!(json[STATS["real_matches"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn matchups_test() {
    use crate::extensions::ugg::constants;
    use constants::STATS;

    if let Ok(json) = UGGDATA.ranking().await {
        assert!(json[STATS["matchups"]].is_array());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn data_test_ranking() {
    use crate::extensions::ugg::constants;
    use constants::STATS;

    if let Ok(json) = UGGDATA.ranking().await {
        let wins = json[STATS["wins"]].as_f64().unwrap();
        let matches = json[STATS["matches"]].as_f64().unwrap();
        assert!(wins / matches < 1.0)
    } else {
        panic!()
    };
}

#[tokio::test]
async fn overview_structure_test() {
    if let Ok(json) = UGGDATA.overview().await {
        assert!(json.is_array());
    } else {
        panic!()
    }
}

#[tokio::test]
async fn runes_test() {
    use crate::extensions::ugg::constants;
    use constants::DATA;

    if let Ok(json) = UGGDATA.overview().await {
        let runes = &json[DATA["perks"]];
        assert!(runes.is_array());
        assert!(runes[4].is_array());
        assert!(runes[4][0].is_i64());
        assert!(runes[2].is_i64());
    } else {
        panic!()
    }
}

#[tokio::test]
async fn items_test() {
    use crate::extensions::ugg::constants;
    use constants::DATA;

    if let Ok(json) = UGGDATA.overview().await {
        let items = &json[DATA["starting_items"]];
        assert!(items.is_array());
        assert!(items[2].is_array());
        assert!(items[2][0].is_i64());
    } else {
        panic!()
    }
}

#[tokio::test]
async fn abilities_test() {
    use crate::extensions::ugg::constants;
    use constants::DATA;

    if let Ok(json) = UGGDATA.overview().await {
        let abilities = &json[DATA["abilities"]];
        assert!(abilities.is_array());
        assert!(abilities[2].is_array());
        assert!(abilities[2][0].is_string());
    } else {
        panic!()
    }
}

#[tokio::test]
async fn shards_test() {
    use crate::extensions::ugg::constants;
    use constants::DATA;

    if let Ok(json) = UGGDATA.overview().await {
        let abilities = &json[DATA["shards"]];
        assert!(abilities.is_array());
        assert!(abilities[2].is_array());
        assert!(abilities[2][0].is_string());
    } else {
        panic!()
    }
}

#[tokio::test]
async fn sort_test() {
    use crate::core::helpers::runes::all_rune_images;

    if let Ok(mut runes) = all_rune_images(8100, 8300, "en_US").await {
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

#[test]
fn abilities_split_test() {
    #[derive(Debug, PartialEq, Eq)]
    struct DummyAbilities<'a> {
        name: &'a str,
        order: Vec<&'a str>,
    }

    let mut maps: [DummyAbilities; 4] = [
        DummyAbilities {
            name: "Q",
            order: Vec::new(),
        },
        DummyAbilities {
            name: "W",
            order: Vec::new(),
        },
        DummyAbilities {
            name: "E",
            order: Vec::new(),
        },
        DummyAbilities {
            name: "R",
            order: Vec::new(),
        },
    ];

    let abilities = [
        "Q", "E", "W", "E", "E", "R", "E", "W", "E", "W", "R", "W", "W", "Q", "Q", "R", "Q", "Q",
    ];

    abilities.iter().for_each(|y| {
        maps.iter_mut().for_each(|ability| {
            if &ability.name == y {
                ability.order.push(y);
            } else {
                ability.order.push("");
            }
        });
    });

    assert!(
        maps == [
            DummyAbilities {
                name: "Q",
                order: [
                    "Q", "", "", "", "", "", "", "", "", "", "", "", "", "Q", "Q", "", "Q", "Q"
                ]
                .to_vec()
            },
            DummyAbilities {
                name: "W",
                order: [
                    "", "", "W", "", "", "", "", "W", "", "W", "", "W", "W", "", "", "", "", ""
                ]
                .to_vec()
            },
            DummyAbilities {
                name: "E",
                order: [
                    "", "E", "", "E", "E", "", "E", "", "E", "", "", "", "", "", "", "", "", ""
                ]
                .to_vec()
            },
            DummyAbilities {
                name: "R",
                order: ["", "", "", "", "", "R", "", "", "", "", "R", "", "", "", "", "R", "", ""]
                    .to_vec()
            }
        ]
    )
}

#[tokio::test]
async fn summoners_test() {
    use crate::extensions::ugg::constants;
    use constants::DATA;

    if let Ok(json) = UGGDATA.overview().await {
        let spell_info = &json[DATA["summoner_spells"]];
        let spells = &spell_info[2];
        println!("{}", spells);
        assert!(spells.is_array());
        assert!(spells[0].is_i64());
        assert!(spells[1].is_i64());
    } else {
        panic!()
    }
}
