#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plugins;
mod shared;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, rune_names, champion_names])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn rune_names(name: String, role: String, rank: String, region: String) -> [Vec<String>; 2] {
    let (rune_names, _rune_ids, _tree_ids) = plugins::ugg::two_dimensional_rune_array(name, role, rank, region).await.unwrap();
    return rune_names;
} 

#[tauri::command]
async fn champion_names() -> Vec<String> {
    let champion_names = shared::helpers::all_champion_names();
    champion_names.await
}
