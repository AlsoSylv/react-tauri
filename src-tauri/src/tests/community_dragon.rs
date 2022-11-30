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