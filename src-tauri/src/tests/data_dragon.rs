use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;

static HYPERCLIENT: Lazy<hyper::Client<HttpsConnector<HttpConnector>>> = Lazy::new(|| {
    let https = HttpsConnector::new();
    hyper::Client::builder().build::<HttpsConnector<HttpConnector>, hyper::Body>(https)
});

#[tokio::test]
async fn champion_full_test() {
    use data_dragon::DataDragon;

    let data_dragon = DataDragon::new(&HYPERCLIENT, None);
    match data_dragon.get_version().await {
        Ok(version) => {
            let json = data_dragon.champ_full("Xayah", &version).await;
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
    use data_dragon::DataDragon;

    let data_dragon = DataDragon::new(&HYPERCLIENT, None);
    match data_dragon.get_version().await {
        Ok(version) => {
            let json = data_dragon.champion_json(&version).await;
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
    use data_dragon::DataDragon;

    let data_dragon = DataDragon::new(&HYPERCLIENT, None);
    match data_dragon.get_version().await {
        Ok(version) => {
            println!("{}", version);
        }
        Err(_) => panic!(),
    }
}

#[tokio::test]
async fn items_test() {
    use data_dragon::DataDragon;

    let data_dragon = DataDragon::new(&HYPERCLIENT, None);
    match data_dragon.get_version().await {
        Ok(version) => {
            let items = data_dragon.item_json(&version).await;
            match items {
                Ok(json) => {
                    if let Some(boots) = json["data"]["1001"]["name"].as_str() {
                        assert!(boots == "Boots");
                    } else {
                        panic!()
                    }
                }
                Err(err) => panic!("{:?}", err),
            }
        }
        Err(_) => panic!(),
    }
}

#[tokio::test]
async fn runes_test() {
    use data_dragon::DataDragon;

    let data_dragon = DataDragon::new(&HYPERCLIENT, None);
    match data_dragon.get_version().await {
        Ok(version) => {
            let runes = data_dragon.rune_json(&version).await;
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
