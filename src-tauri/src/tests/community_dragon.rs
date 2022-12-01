#[tokio::test]
async fn champ_basic_test() {
    use crate::core::community_dragon::CommunityDragon;

    let community_dragon = CommunityDragon::new_with_client("en_US");
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

    let community_dragon = CommunityDragon::new_with_client("en_US");
    if let Ok(champ_full) = community_dragon.champs_full(498).await {
        assert!(champ_full["name"] == "Xayah");
        assert!(champ_full["alias"] == "Xayah");
    } else {
        panic!()
    }
}

#[tokio::test]
async fn runes_test() {
    use crate::core::community_dragon::CommunityDragon;

    let community_dragon = CommunityDragon::new_with_client("en_US");
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

    let community_dragon = CommunityDragon::new_with_client("en_US");
    if let Ok(runes) = community_dragon.runes_style().await {
        assert!(runes.schema_version == 2);
        assert!(runes.styles[0].id == 8400);
        assert!(!runes.styles[0].is_advanced);
    } else {
        panic!()
    }
}
