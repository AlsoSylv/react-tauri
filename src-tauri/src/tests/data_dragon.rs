#[tokio::test]
async fn champion_full_test() {
    use crate::core::data_dragon::structs::DataDragon;

    let data_dragon = DataDragon::new(None).await;
    match data_dragon {
        Ok(data_dragon) => {
            let json = data_dragon.champ_full(String::from("Xayah")).await;
            match json {
                Ok(json) => {
                    if let Some(id) = json.data["Xayah"]["key"].as_str() {
                        assert!(id == "498");
                    } else {
                        panic!()
                    };
                }
                Err(_) => panic!(),
            }
        }
        Err(_) => panic!(),
    }
}

#[tokio::test]
async fn champion_json_test() {
    use crate::core::data_dragon::structs::DataDragon;

    let data_dragon = DataDragon::new(None).await;
    match data_dragon {
        Ok(data_dragon) => {
            let json = data_dragon.champion_json().await;
            match json {
                Ok(json) => {
                    assert!(json.data["Xayah"].key == String::from("498"))
                }
                Err(_) => panic!(),
            }
        }
        Err(_) => panic!(),
    }
}

#[tokio::test]
async fn new_test() {
    use crate::core::data_dragon::structs::DataDragon;

    let data_dragon = DataDragon::new(None).await;
    match data_dragon {
        Ok(data_dragon) => {
            println!("{}", data_dragon.version);
            assert!(data_dragon.language == String::from("en_US"));
        }
        Err(_) => panic!(),
    }
}

#[tokio::test]
async fn items_test() {
    use crate::core::data_dragon::structs::DataDragon;

    let data_dragon = DataDragon::new(None).await;
    match data_dragon {
        Ok(data_dragon) => {
            let items = data_dragon.item_json().await;
            match items {
                Ok(json) => {
                    if let Some(boots) = json["data"]["1001"]["name"].as_str() {
                        assert!(boots == "Boots");
                    } else {
                        panic!()
                    }
                }
                Err(_) => panic!(),
            }
        }
        Err(_) => panic!(),
    }
}

#[tokio::test]
async fn runes_test() {
    use crate::core::data_dragon::structs::DataDragon;

    let data_dragon = DataDragon::new(None).await;
    match data_dragon {
        Ok(data_dragon) => {
            let runes = data_dragon.runes_json().await;
            match runes {
                Ok(json) => {
                    let domination = &json[0];
                    assert!(domination.id == 8100);
                    assert!(domination.key == String::from("Domination"));

                    let key_stones = &domination.slots[0].runes;
                    assert!(key_stones[0].id == 8112);
                    assert!(key_stones[0].key == String::from("Electrocute"));
                }
                Err(_) => panic!(),
            }
        }
        Err(_) => panic!(),
    }
}
