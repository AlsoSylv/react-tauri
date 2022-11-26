use serde_json::Value;

use crate::errors;
use errors::{ErrorMap, UGGDataError};

use super::{constants, structs};

use constants::DATA;
use structs::{Shard, Shards};

impl structs::Data {
    /// Returns stat shards from the UGG API
    ///
    /// This requires Community Dragon to work
    /// without being hardcoded
    pub async fn shard_tuple(&self, request: Result<Value, ErrorMap>) -> Result<Shards, ErrorMap> {
        // TODO: Use Community Dragon to get shard data
        let armor = Shard::create(
            "Armor",
            5002,
            "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsArmorIcon.png",
        );

        let magic_resist = Shard::create(
            "Magic Resist", 
            5003,
            "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsMagicResIcon.png"
        );

        let health = Shard::create(
            "Health", 
            5001,
            "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsHealthScalingIcon.png"
        );

        let adaptive_force = Shard::create(
            "Adaptive Force",
            5008,
            "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsAdaptiveForceIcon.png"
        );

        let attack_speed = Shard::create(
            "Attack Speed", 
            5005,
            "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsAttackSpeedIcon.png"
        );

        let ability_haste = Shard::create(
            "Ability Haste", 
            5007,
            "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsCDRScalingIcon.png"
        );

        let mut shards: Shards = Shards {
            row_one: [adaptive_force.clone(), attack_speed, ability_haste],
            row_two: [adaptive_force, armor.clone(), magic_resist.clone()],
            row_three: [health, armor, magic_resist],
        };

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
