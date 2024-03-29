use data_dragon::DataDragon;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;

use crate::{extensions::ugg::Data, frontend_types::ChampionNames};

static HYPERCLIENT: Lazy<hyper::Client<HttpsConnector<HttpConnector>>> = Lazy::new(|| {
    let https = HttpsConnector::new();
    hyper::Client::builder().build::<HttpsConnector<HttpConnector>, hyper::Body>(https)
});

static DATADRAGON: Lazy<DataDragon> = Lazy::new(|| DataDragon::new(&HYPERCLIENT, None));

static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

static NAME: Lazy<ChampionNames> = Lazy::new(|| ChampionNames::new("", "", 498, None));

static UGGDATA: Lazy<Data> = Lazy::new(|| {
    Data::new(
        &NAME,
        "3",
        "Platinum Plus",
        "World",
        Some("en_US"),
        &DATADRAGON,
        &CLIENT,
    )
});

#[tokio::test]
async fn ranking_structure_test() {
    match UGGDATA.ranking().await {
        Ok(_) => (),
        Err(err) => panic!("{:?}", err),
    }
}

#[tokio::test]
async fn wins_test() {
    if let Ok(json) = UGGDATA.overview().await {
        let a = json.winrate.unwrap();
        a.wins.unwrap();
        a.matches.unwrap();
    } else {
        panic!()
    };
}

#[tokio::test]
async fn matches_test() {
    if let Ok(json) = UGGDATA.ranking().await {
        json.matches.unwrap();
    } else {
        panic!()
    };
}

#[tokio::test]
async fn rank_test() {
    if let Ok(json) = UGGDATA.ranking().await {
        json.rank.unwrap();
    } else {
        panic!()
    };
}

#[tokio::test]
async fn total_rank_test() {
    if let Ok(json) = UGGDATA.ranking().await {
        json.total_rank.unwrap();
    } else {
        panic!()
    };
}

#[tokio::test]
async fn bans_test() {
    if let Ok(json) = UGGDATA.ranking().await {
        json.bans.unwrap();
    } else {
        panic!()
    };
}

#[tokio::test]
async fn total_matches_test() {
    if let Ok(json) = UGGDATA.ranking().await {
        json.total_matches.unwrap();
    } else {
        panic!()
    };
}

#[tokio::test]
async fn real_matches_test() {
    if let Ok(json) = UGGDATA.ranking().await {
        json.real_matches.unwrap();
    } else {
        panic!()
    };
}

#[tokio::test]
async fn matchups_test() {
    if let Ok(json) = UGGDATA.ranking().await {
        assert!((json.matchups[0][1] as f64 / json.matchups[0][1] as f64) <= 1.0);
    } else {
        panic!()
    };
}

#[tokio::test]
async fn data_test_ranking() {
    if let Ok(json) = UGGDATA.ranking().await {
        let wins = json.wins.unwrap();
        let matches = json.matches.unwrap();
        assert!(wins / matches < 1.0)
    } else {
        panic!()
    };
}

#[tokio::test]
async fn overview_structure_test() {
    match UGGDATA.overview().await {
        Ok(_) => (),
        Err(err) => panic!("{:?}", err),
    }
}

#[tokio::test]
async fn runes_test() {
    if let Ok(json) = UGGDATA.overview().await {
        let runes = &json.perks;
        runes.rune_ids.as_ref().unwrap();
    } else {
        panic!()
    }
}

#[tokio::test]
async fn items_test() {
    if let Ok(json) = UGGDATA.overview().await {
        let items = &json.starting_items;
        assert!(items.wins.unwrap() > 100.0);
    } else {
        panic!()
    }
}

#[tokio::test]
async fn abilities_test() {
    if let Ok(json) = UGGDATA.overview().await {
        let abilities = &json.abilities;
        assert!(abilities.is_array());
        assert!(abilities[2].is_array());
        assert!(abilities[2][0].is_string());
    } else {
        panic!()
    }
}

#[tokio::test]
async fn shards_test() {
    if let Ok(json) = UGGDATA.overview().await {
        let abilities = json.shards;
        abilities.unwrap();
    } else {
        panic!()
    }
}

#[tokio::test]
async fn sort_test() {
    use crate::core::helpers::runes::all_rune_images;

    if let Ok(mut runes) = all_rune_images(8100, 8300, Some("en_US"), &CLIENT, &DATADRAGON).await {
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
    if let Ok(json) = UGGDATA.overview().await {
        let spell_info = &json.summoner_spells;
        let _ = &spell_info.spells.as_ref().unwrap();
    } else {
        panic!()
    }
}
