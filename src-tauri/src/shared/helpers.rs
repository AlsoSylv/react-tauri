use cached::proc_macro::cached;
use tauri::regex::Regex;
use serde_json::{Value, json};
use once_cell::sync::Lazy;

use crate::{Active, RuneImages, PrimaryTree, SecondaryTree};

use super::data_dragon::{self};

pub async fn champion_name_sanitizer(name: String, title: bool) -> String {
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
        .unwrap_or_else(|| champ_name[0..1].to_uppercase() + &champ_name[1..].to_lowercase());
    
    if title == true {
        return champ_name
    } else {
        return champ_name.to_lowercase();
    }
}

#[cached(size = 25, result = true)]
pub async fn champion_id(name: String) -> Result<i64, i64> { 
    let champion_name = format!("{}", champion_name_sanitizer(name.clone(), true).await);
    println!("{}", champion_name);
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

pub async fn all_rune_images(tree_id_one: i64, tree_id_two: i64) -> Result<RuneImages, i64> {
    let request = data_dragon::runes_json().await;
    let mut tree_one_names: PrimaryTree = PrimaryTree { slot_one: Vec::new(), slot_two: Vec::new(), slot_three: Vec::new(), slot_four: Vec::new() };
    let mut tree_two_names: SecondaryTree = SecondaryTree { slot_one: Vec::new(), slot_two: Vec::new(), slot_three: Vec::new() };
    match request {
        Ok(json) => {
            for rune in json.iter() {
                if &rune.id == &tree_id_one {
                    for (position, slots) in rune.slots.iter().enumerate() {
                        for runes in &slots.runes {
                            match position {
                                0 => tree_one_names.slot_one.push(Active {name: runes.name.clone(), image: "http://ddragon.leagueoflegends.com/cdn/img/".to_string() + &runes.icon.clone(), active: false,  id: runes.id}),
                                1 => tree_one_names.slot_two.push(Active {name: runes.name.clone(), image: "http://ddragon.leagueoflegends.com/cdn/img/".to_string() + &runes.icon.clone(), active: false,  id: runes.id}),
                                2 => tree_one_names.slot_three.push(Active {name: runes.name.clone(), image: "http://ddragon.leagueoflegends.com/cdn/img/".to_string() + &runes.icon.clone(), active: false,  id: runes.id}),
                                3 => tree_one_names.slot_four.push(Active {name: runes.name.clone(), image: "http://ddragon.leagueoflegends.com/cdn/img/".to_string() + &runes.icon.clone(), active: false,  id: runes.id}),
                                _ => unreachable!()
                            }

                        }
                    }
                } else if &rune.id == &tree_id_two {
                    for i in 1..4 {
                        let slots = &rune.slots[i];
                        for runes in &slots.runes {
                            match i {
                                1 => tree_two_names.slot_one.push(Active {name: runes.name.clone(), image: "http://ddragon.leagueoflegends.com/cdn/img/".to_string() + &runes.icon.clone(), active: false,  id: runes.id}),
                                2 => tree_two_names.slot_two.push(Active {name: runes.name.clone(), image: "http://ddragon.leagueoflegends.com/cdn/img/".to_string() + &runes.icon.clone(), active: false,  id: runes.id}),
                                3 => tree_two_names.slot_three.push(Active {name: runes.name.clone(), image: "http://ddragon.leagueoflegends.com/cdn/img/".to_string() + &runes.icon.clone(), active: false,  id: runes.id}),
                                _ => unreachable!()
                            }
                        }
                    }
                }
            }
            let rune_names = RuneImages { primary_runes: tree_one_names, secondary_runes: tree_two_names };
            Ok(rune_names)
        }
        Err(err) => Err(err)
    }
}
