use cached::proc_macro::cached;
use tauri::regex::Regex;
use serde_json::{Value, json};

use super::data_dragon::{self};

pub async fn champion_name_sanitizer(name: String, title: bool) -> String {
    let mut champ_name = name.clone();
    champ_name = champ_name.split("&").collect::<Vec<&str>>()[0].to_owned();
    let champ_split = champ_name.split(" ").collect::<Vec<&str>>();
    if champ_split.len() > 1 {
        champ_name = champ_name.split(" ").collect::<Vec<&str>>()[0].to_owned() + champ_name.split(" ").collect::<Vec<&str>>().to_owned()[1];
    }
    println!("{:?}", champ_name.split(" ").collect::<Vec<&str>>());
    champ_name = Regex::new(r"\W").unwrap().replace_all(&champ_name, r"").to_string();
    champ_name.retain(|c| !c.is_whitespace());
    if title == true {
        return champ_name
    } else {
        return champ_name.to_lowercase();
    }
}

#[cached(size = 25, result = true)]
pub async fn champion_id(name: String) -> Result<i64, reqwest::Error> { 
    let champion_name = format!("{}", champion_name_sanitizer(name.clone(), true).await);
    let champion_json = data_dragon::champion_json().await;
    let champion_id: &i64 = &champion_json.data[&champion_name].key.parse().unwrap();
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
    let champion_json = data_dragon::champion_json().await;
    for (_xy, y) in champion_json.data.iter() {
        champions.push(y.clone().name);
    }
    return champions;
}