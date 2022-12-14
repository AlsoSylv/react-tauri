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
        assert!(champ_full["name"] == "Xayah");
        assert!(champ_full["alias"] == "Xayah");
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