#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


mod plugins;
mod shared;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![rune_names, champion_names, win_rate, shard_names])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RuneImages {
    primary_runes: Runes,
    secondary_runes: Runes
}

type Runes = Vec<Active>;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Active  {
    name: String,
    image: String,
    active: bool,
}

#[tauri::command]
async fn rune_names(name: String, role: String, rank: String, region: String) -> Result<RuneImages, i64> {
    // TOOD: This can be none if you get data specific enough, I need to handle that 
    let rune_match = plugins::ugg::rune_tuple(name, role, rank, region).await;
    let mut rune_images_tree_one: Runes = vec![];
    let mut rune_iamges_tree_two: Runes = vec![];
    match rune_match {
        Ok((rune_names, _rune_ids, tree_ids, urls)) => {
            let request = shared::helpers::all_rune_images(tree_ids[0], tree_ids[1]).await;
            match request {
                Ok((all_urls, names)) => {
                    println!("{:#?}", names);
                    println!("{:#?}", rune_names);
                    for (name_position, name) in names.iter().enumerate() {
                        for (string_position, string) in name.iter().enumerate() {
                            for y in &rune_names[name_position] {
                                if y == string {
                                    rune_images_tree_one.push(Active { name: names[name_position][string_position].clone(), image: all_urls[name_position][string_position].clone(), active: true });
                                } else {
                                    let active = Active {name: names[name_position][string_position].clone(), image: all_urls[name_position][string_position].clone(), active: true};
                                    let inactive = Active {name: names[name_position][string_position].clone(), image: all_urls[name_position][string_position].clone(), active: false};
                                    if !rune_images_tree_one.contains(&active) && !rune_images_tree_one.contains(&inactive) {
                                        rune_images_tree_one.push(Active { name: names[name_position][string_position].clone(), image: all_urls[name_position][string_position].clone(), active: false })
                                    }
                                }
                            }
                        }
                    }
                    println!("{:?}", rune_images_tree_one);
                    Ok(RuneImages { primary_runes: rune_images_tree_one, secondary_runes: rune_iamges_tree_two })
                }
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}

#[tauri::command]
async fn win_rate(name: String, role: String, rank: String, region: String) -> Result<String, i64> {
    let request = plugins::ugg::winrate(name, role, rank, region).await;
    match request {
    Ok(winrate) => Ok(winrate),
    Err(err) => Err(err)
    }
}

#[tauri::command]
async fn champion_names() -> Result<Vec<String>, i64> {
    let request = shared::helpers::all_champion_names().await;
    match request {
        Ok(names) => {
            Ok(names)
        }
        Err(err) => Err(err)
    }
}

#[tauri::command]
async fn shard_names(name: String, role: String, rank: String, region: String) -> Result<[String; 3], i64> {
    let shards = plugins::ugg::shard_tuple(name, role, rank, region).await;
    match shards {
        Ok((names, _ids)) => {
            Ok(names)
        }
        Err(err) => Err(err)
    }
}
