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

#[tauri::command]
async fn rune_names(name: String, role: String, rank: String, region: String) -> Result<[Vec<String>; 2], String> {
    // TOOD: This can be none if you get data specific enough, I need to handle that 
    let rune_match = plugins::ugg::rune_tuple(name, role, rank, region).await;
    match rune_match {
        Ok((rune_names, _rune_ids, _tree_ids)) => {
            Ok(rune_names)
        },
        Err(_) => Err("Data does not exist".to_string())
    }
}

#[tauri::command]
async fn win_rate(name: String, role: String, rank: String, region: String) -> Result<String, String> {
    let winrate = plugins::ugg::winrate(name, role, rank, region).await;
    Ok(winrate)
}

#[tauri::command]
async fn champion_names() -> Vec<String> {
    let champion_names = shared::helpers::all_champion_names();
    champion_names.await
}

#[tauri::command]
async fn shard_names(name: String, role: String, rank: String, region: String) -> Result<[String; 3], String> {
    let shards = plugins::ugg::shard_tuple(name, role, rank, region).await;
    match shards {
        Ok((names, _ids)) => {
            Ok(names)
        }
        Err(_) => Err("Data does not exist".to_owned())
    }
}
