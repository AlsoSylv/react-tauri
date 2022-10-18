use cached::proc_macro::cached;
use tauri::regex::{Regex};
use serde_json::{Value, json};

use super::data_dragon::{self};

async fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

async fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

pub async fn champion_name_sanitizer(name: String, title: bool) -> String {
        let re = Regex::new(r"\W").unwrap();
        let mut champion_name = re.replace_all(&name, r"").to_lowercase();
        remove_whitespace(&mut champion_name).await;
        if title == true {
            let champion_name = capitalize_first_letter(&champion_name).await;
            return champion_name
        } else {
            return champion_name
    }
}

#[cached(size = 25, result = true)]
pub async fn champion_id(name: String) -> Result<i64, reqwest::Error> { 
    let champion_name = champion_name_sanitizer(name.clone(), true).await;
    //let champion_name = capitalize_first_letter(&name.clone()).await;
    let champion_json: Value = data_dragon::champion_json().await;
    let champion_id: &i64 = &champion_json["data"][format!("{champion_name}")]["key"].as_str().unwrap().parse().unwrap();
    Ok(champion_id.to_owned())
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
pub async fn all_champion_names() -> Vec<String> {
    let mut champions = Vec::new();
    let champion_json: Value = data_dragon::champion_json().await;
    for (_xy, y) in champion_json["data"].as_object().unwrap() {
        champions.push(y["name"].to_string());
    }
    return champions;
}