#[tokio::test]
async fn ranking_structure_test() {
    use super::json;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        println!("{:#?}", json);
        assert!(json.is_array());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn wins_test() {
    use super::{json, constants};
    use constants::STATS;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        assert!(json[STATS["wins"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn matches_test() {
    use super::{json, constants};
    use constants::STATS;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        assert!(json[STATS["matches"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn rank_test() {
    use super::{json, constants};
    use constants::STATS;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        assert!(json[STATS["rank"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn total_rank_test() {
    use super::{json, constants};
    use constants::STATS;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        assert!(json[STATS["total_rank"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn bans_test() {
    use super::{json, constants};
    use constants::STATS;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        assert!(json[STATS["bans"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn total_matches_test() {
    use super::{json, constants};
    use constants::STATS;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        assert!(json[STATS["total_matches"]].is_f64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn real_matches_test() {
    use super::{json, constants};
    use constants::STATS;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        assert!(json[STATS["real_matches"]].is_i64());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn matchups_test() {
    use super::{json, constants};
    use constants::STATS;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        assert!(json[STATS["matchups"]].is_array());
    } else {
        panic!()
    };
}

#[tokio::test]
async fn data_test_ranking() {
    use super::{json, constants};
    use constants::STATS;

    if let Ok(json) = json::ranking(
        &498, "ADC", "Platinum Plus", "World", "en_US"
    ).await {
        let wins = json[STATS["wins"]].as_f64().unwrap();
        let matches = json[STATS["matches"]].as_f64().unwrap();
        assert!(wins / matches < 1.0)
    } else {
        panic!()
    };
}
