use super::{structs::{self, Shard, Shards}, json::overview, constants::DATA};

impl structs::Data {
    pub async fn shard_tuple(&self) -> Result<Shards, i64> {
        let armor = Shard::create(
            "Armor", 
            5002, 
            "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsArmorIcon.png"
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
    
        let attack_speed = Shard {
            name: "Attack Speed".to_owned(),
            id: 5005,
            image: "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsAttackSpeedIcon.png".to_owned(),
            active: false
        };
    
        let ability_haste = Shard {
            name: "Ability Haste".to_owned(),
            id: 5007,
            image: "http://ddragon.leagueoflegends.com/cdn/img/perk-images/StatMods/StatModsCDRScalingIcon.png".to_owned(),
            active: false
        };
    
        let shards: Shards = Shards {
            row_one: [adaptive_force.clone(), attack_speed, ability_haste],
            row_two: [adaptive_force, armor.clone(), magic_resist.clone()],
            row_three: [health, armor, magic_resist],
        };
    
        let mut mutable_shards = shards.clone();
    
        let request = overview(self.name.clone(), self.role.clone(), self.rank.clone(), self.region.clone()).await;
        match request {
            Ok(json) => {
                let active_shards = json[DATA["shards"]][2].as_array();
                match active_shards {
                    Some(active_shards) => {
                        for (y, shard) in shards.row_one.iter().enumerate() {
                            if shard.id.to_string() == active_shards[0] {
                                mutable_shards.row_one[y] = Shard {
                                    name: shard.name.clone(),
                                    id: shard.id,
                                    image: shard.image.clone(),
                                    active: true,
                                }
                            }
                        }
    
                        for (y, shard) in shards.row_two.iter().enumerate() {
                            if shard.id.to_string() == active_shards[1] {
                                mutable_shards.row_two[y] = Shard {
                                    name: shard.name.clone(),
                                    id: shard.id,
                                    image: shard.image.clone(),
                                    active: true,
                                }
                            }
                        }
    
                        for (y, shard) in shards.row_three.iter().enumerate() {
                            if shard.id.to_string() == active_shards[2] {
                                mutable_shards.row_three[y] = Shard {
                                    name: shard.name.clone(),
                                    id: shard.id,
                                    image: shard.image.clone(),
                                    active: true,
                                }
                            }
                        }
    
                        Ok(mutable_shards)
                    }
                    None => Err(202),
                }
            }
            Err(err) => Err(err),
        }
    }
}