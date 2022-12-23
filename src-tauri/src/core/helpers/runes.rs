use crate::{
    core::{community_dragon::CommunityDragon, data_dragon::DataDragon},
    errors::{CommunityDragonError, ErrorMap},
    frontend_types,
};

use frontend_types::{Active, PrimaryTree, RuneImages, SecondaryTree};

/// Transforms the Raw DataDragon runesReforged.json into a more manageable format
/// that only contains the runes for the specified IDs
///
///  # Examples
///
/// ```
/// let runes = all_rune_images(8100, 8200, "en_US");
/// ```
pub async fn all_rune_images(
    tree_id_one: i64,
    tree_id_two: i64,
    language: &str,
) -> Result<RuneImages, ErrorMap> {
    if let Ok(data_dragon) = DataDragon::new(Some(language)).await {
        let request = data_dragon.runes_json().await;
        let mut tree_one_names = PrimaryTree::new();
        let mut tree_two_names = SecondaryTree::new();
        let tree_one_array = tree_one_names.as_array_mut();
        let tree_two_array = tree_two_names.as_array_mut();

        match request {
            Ok(json) => {
                for rune in json.iter() {
                    if rune.id == tree_id_one {
                        for (position, slots) in rune.slots.iter().enumerate() {
                            for runes in &slots.runes {
                                tree_one_array[position].push(Active::new(
                                    &runes.name,
                                    format!(
                                        "http://ddragon.leagueoflegends.com/cdn/img/{}",
                                        &runes.icon
                                    ),
                                    runes.id,
                                    format!("/{0}/{1}.png", rune.key, runes.key),
                                    &runes.long_desc,
                                ));
                            }
                        }
                    } else if rune.id == tree_id_two {
                        for i in 1..4 {
                            let slot = &rune.slots[i];
                            for runes in &slot.runes {
                                tree_two_array[i - 1].push(Active::new(
                                    &runes.name,
                                    format!(
                                        "http://ddragon.leagueoflegends.com/cdn/img/{}",
                                        &runes.icon
                                    ),
                                    runes.id,
                                    format!("/{0}/{1}.png", rune.key, runes.key),
                                    &runes.long_desc,
                                ));
                            }
                        }
                    }
                }
                Ok(RuneImages {
                    primary_runes: tree_one_names,
                    secondary_runes: tree_two_names,
                })
            }
            Err(err) => Err(ErrorMap::DataDragonErrors(err)),
        }
    } else {
        let runes = community_dragon_all_rune_images(tree_id_one, tree_id_two, language).await;
        match runes {
            Ok(runes) => Ok(runes),
            Err(err) => Err(ErrorMap::CommunityDragonErrors(err)),
        }
    }
}

pub async fn community_dragon_all_rune_images(
    tree_id_one: i64,
    tree_id_two: i64,
    language: &str,
) -> Result<RuneImages, CommunityDragonError> {
    let community_dragon = CommunityDragon::new(language);
    let runes_style = community_dragon.runes_style().await;
    let rune = community_dragon.runes().await;

    let mut tree_one_names = PrimaryTree::new();
    let mut tree_two_names = SecondaryTree::new();
    let mut tree_one_array = tree_one_names.as_array_mut();
    let tree_two_array = tree_two_names.as_array_mut();

    match runes_style {
        Ok(rune_style_json) => match rune {
            Ok(rune_json) => {
                for rune in rune_style_json.styles.iter() {
                    if rune.id == tree_id_one {
                        for (i, array) in tree_one_array.iter_mut().enumerate() {
                            let slot = &rune.slots[i];
                            for rune_ids in slot.perks.iter() {
                                for rune_specifics in rune_json.iter() {
                                    if rune_ids == &rune_specifics.id {
                                        if let Some(path_location) =
                                            rune_specifics.icon_path.find("/v1/")
                                        {
                                            array.push(Active::new(
                                                &rune_specifics.name,
                                                format!(
                                                    "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default{}",
                                                    rune_specifics.icon_path.split_at(path_location).1.to_lowercase()
                                                ),
                                                rune_specifics.id,
                                                format!("/{0}/{1}.png", rune_specifics.name, rune_specifics.name),
                                                &rune_specifics.long_desc,
                                            ));
                                        };
                                    }
                                }
                            }
                        }
                    } else if rune.id == tree_id_two {
                        for i in 1..4 {
                            let slot = &rune.slots[i];
                            for rune_ids in slot.perks.iter() {
                                for rune_specifics in rune_json.iter() {
                                    if rune_ids == &rune_specifics.id {
                                        if let Some(path_location) =
                                            rune_specifics.icon_path.find("/v1/")
                                        {
                                            tree_two_array[i - 1].push(Active::new(
                                                &rune_specifics.name,
                                                format!(
                                                    "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default{}",
                                                    rune_specifics.icon_path.split_at(path_location).1.to_lowercase()
                                                ),
                                                rune_specifics.id,
                                                format!("/{0}/{1}.png", rune_specifics.name, rune_specifics.name),
                                                &rune_specifics.long_desc,
                                            ));
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(RuneImages {
                    primary_runes: tree_one_names,
                    secondary_runes: tree_two_names,
                })
            }
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
