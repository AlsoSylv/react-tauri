use std::time::Instant;

use cached::proc_macro::cached;
use tauri::regex::Regex;
use serde_json::{Value, json};
use once_cell::sync::Lazy;

use super::data_dragon::{self};

pub async fn champion_name_sanitizer(name: String, title: bool) -> String {
    let now = Instant::now();
    static CHARACTER_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\W").unwrap());
    
    let champ_name = name;
    
    // Nunu & Willump
    let champ_name = champ_name.split_once("&")
        .map(|(f, _)| f)
        .unwrap_or(&champ_name);
    
    // A-Za-z0-9_
    let champ_name = CHARACTER_NAME_REGEX.replace_all(&champ_name, r"");
    
    // Renata Glasc
    let champ_name = champ_name.split_once(" ")
        .map(|(f, s)| f.to_string() + s)
        .unwrap_or_else(|| champ_name.to_string());
    
    if title == true {
        println!("{}", now.elapsed().as_millis());
        return champ_name
    } else {
        return champ_name.to_lowercase();
    }
}

#[cached(size = 25, result = true)]
pub async fn champion_id(name: String) -> Result<i64, i64> { 
    let champion_name = format!("{}", champion_name_sanitizer(name.clone(), true).await);
    let request = data_dragon::champion_json().await; //: &i64 
    match request {
        Ok(json) => {
            Ok(json.data[&champion_name].key.parse().unwrap())
        }
        Err(err) => Err(err)
    }
}

pub async fn create_rune_page(name: String, primary_id: String, secondary_id: String, selected_perks: [i64; 9]) -> Value {
    let rune_page = json!({
        "name": name,
        "primaryStyleId": primary_id, 
        "subStyleId": secondary_id,
        "selectedPerkIds": selected_perks
    });
    return rune_page;
}

#[cached]
pub async fn all_champion_names() -> Result<Vec<String>, i64> {
    let mut champions = Vec::new();
    let request = data_dragon::champion_json().await;
    match request {
        Ok(json) => {
            for (_xy, y) in json.data.iter() {
                champions.push(y.clone().name);
            }
            Ok(champions)
        }
        Err(err) => Err(err)
    }
}