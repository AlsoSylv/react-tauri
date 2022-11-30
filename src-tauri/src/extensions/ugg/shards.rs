use serde_json::Value;

use crate::{core::community_dragon::CommunityDragon, errors};
use errors::{ErrorMap, UGGDataError};

use super::{constants, structs};

use constants::DATA;
use structs::{Shard, Shards};

impl super::Data {
    /// Returns stat shards from the UGG API
    ///
    /// This requires Community Dragon to work
    /// without being hardcoded
    pub async fn shard_tuple(&self, request: Result<Value, ErrorMap>) -> Result<Shards, ErrorMap> {
        let mut shards = new_shards();
        let community_dragon = CommunityDragon::new_with_client(&self.lang);
        let rune_json = community_dragon.runes().await;
        match rune_json {
            Ok(json) => {
                for n in shards.as_array_mut() {
                    for i in n {
                        for x in &json {
                            if x.id == i.id {
                                i.name = x.name.clone();
                                i.description = x.tooltip.clone();
                            }
                        }
                    }
                }

                match request {
                    Ok(json) => {
                        let active_shards = json[DATA["shards"]][2].as_array();
                        match active_shards {
                            Some(active_shards) => {
                                sort_shards(&mut shards, active_shards);
                                Ok(shards)
                            }
                            None => Err(ErrorMap::UGGError(UGGDataError::OverviewConnect)),
                        }
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(ErrorMap::CommunityDragonErrors(err)),
        }
        // TODO: Use Community Dragon to get shard data
    }
}

fn sort_shards(shards: &mut Shards, active_shards: &[Value]) {
    shards
        .as_array_mut()
        .iter_mut()
        .enumerate()
        .for_each(|(pos, shard_array)| {
            shard_array.iter_mut().for_each(|shard| {
                let string_id = shard.id.to_string();
                if string_id == active_shards[pos] {
                    shard.active = true;
                }
            });
        })
}

/// Returns shards because i can't figure out how to generate this list with zero order
fn new_shards() -> Shards {
    let armor = Shard::new(5002, "StatModsArmorIcon.png");
    let magic_resist = Shard::new(5003, "StatModsMagicResIcon.png");
    let health = Shard::new(5001, "StatModsHealthScalingIcon.png");
    let adaptive_force = Shard::new(5008, "StatModsAdaptiveForceIcon.png");
    let attack_speed = Shard::new(5005, "StatModsAttackSpeedIcon.png");
    let ability_haste = Shard::new(5007, "StatModsCDRScalingIcon.png");
    Shards {
        row_one: [adaptive_force.clone(), attack_speed, ability_haste],
        row_two: [adaptive_force, armor.clone(), magic_resist.clone()],
        row_three: [health, armor, magic_resist],
    }
}
