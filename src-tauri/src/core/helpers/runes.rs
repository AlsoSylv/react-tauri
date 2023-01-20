use crate::{
    core::{
        community_dragon::{structs::Style, CommunityDragon},
        data_dragon::{structs::RuneTree, DataDragon},
    },
    errors::{CommunityDragonError, ErrorMap, Errors},
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
    match DataDragon::new(Some(language)).await {
        Ok(data_dragon) => {
            let mut tree_one_names = PrimaryTree::new();
            let mut tree_two_names = SecondaryTree::new();

            match data_dragon.runes_json().await {
                Ok(json) => {
                    for rune in json {
                        if rune.id == tree_id_one {
                            split_trees_data_dragon(0, 4, &rune, &mut tree_one_names.as_array_mut())
                        } else if rune.id == tree_id_two {
                            split_trees_data_dragon(1, 4, &rune, &mut tree_two_names.as_array_mut())
                        }
                    }
                    Ok(RuneImages {
                        tree_one: tree_id_one,
                        tree_two: tree_id_two,
                        primary_runes: tree_one_names,
                        secondary_runes: tree_two_names,
                    })
                }
                Err(err) => Err(ErrorMap::DataDragonErrors(err)),
            }
        }
        Err(err) => {
            if err.is_connection() {
                Err(ErrorMap::DataDragonErrors(err))
            } else {
                let runes =
                    community_dragon_all_rune_images(tree_id_one, tree_id_two, language).await;
                match runes {
                    Ok(runes) => Ok(runes),
                    Err(err) => Err(ErrorMap::CommunityDragonErrors(err)),
                }
            }
        }
    }
}

fn split_trees_data_dragon(
    start: usize,
    end: usize,
    rune: &RuneTree,
    active_tree: &mut [&mut Vec<Active>],
) {
    for i in start..end {
        for runes in &rune.slots[i].runes {
            active_tree[i - start].push(Active::new(
                &runes.name,
                format!("http://ddragon.leagueoflegends.com/cdn/img/{}", &runes.icon),
                runes.id,
                format!("/{0}/{1}.png", rune.key, runes.key),
                &runes.long_desc,
            ));
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
    let rune = generate_perks(&community_dragon).await;

    let mut tree_one_names = PrimaryTree::new();
    let mut tree_two_names = SecondaryTree::new();

    match runes_style {
        Ok(rune_style_json) => match rune {
            Ok(rune_json) => {
                for rune in rune_style_json.styles.iter() {
                    if rune.id == tree_id_one {
                        split_trees_community_dragon(0, 4, rune, &rune_json, &mut tree_one_names.as_array_mut())
                    } else if rune.id == tree_id_two {
                        split_trees_community_dragon(1, 4, rune, &rune_json, &mut tree_two_names.as_array_mut())
                    }
                }
                Ok(RuneImages {
                    tree_one: tree_id_one,
                    tree_two: tree_id_two,
                    primary_runes: tree_one_names,
                    secondary_runes: tree_two_names,
                })
            }
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn split_trees_community_dragon(
    start: usize,
    end: usize,
    styles: &Style,
    maybe_active: &Vec<Active>,
    active_tree: &mut [&mut Vec<Active>],
) {
    for i in start..end {
        let slot = &styles.slots[i];
        for id in &slot.perks {
            for details in maybe_active {
                if *id == details.id {
                    active_tree[i - start].push(details.clone());
                }
            }
        }
    }
}

async fn generate_perks(
    community_dragon: &CommunityDragon,
) -> Result<Vec<Active>, CommunityDragonError> {
    let rune = community_dragon.runes().await?;
    let mut runes: Vec<Active> = Vec::new();
    for details in rune {
        if let Some(path_location) = details.icon_path.find("/v1/") {
            runes.push(Active::new(
                &details.name,
                format!(
                    "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default{}",
                    details.icon_path.split_at(path_location).1.to_lowercase()
                ),
                details.id,
                format!("/{0}/{1}.png", details.name, details.name),
                &details.long_desc,
            ));
        };
    }
    Ok(runes)
}
