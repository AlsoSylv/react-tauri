use crate::{
    errors::ErrorMap,
    frontend_types::{Spell, SummonerSpellInfo}, core::community_dragon::CommunityDragon,
};

use super::{
    structs::{Overview, SummonerSpells},
    Data,
};

impl Data<'_> {
    pub async fn summoners(
        &self,
        request: &Result<Overview, ErrorMap>,
    ) -> Result<SummonerSpellInfo, ErrorMap> {
        // let data_dragon = DataDragon::new(Some(&self.lang)).await;
        match request {
            Ok(json) => {
                // spell_info[1] -> Wins with combo
                // spell_info[0] -> Matches with combo
                // spell_info[2] -> Array of spells
                let spell_info = &json.summoner_spells;
                let Some(games) = &spell_info.wins else {
                    panic!()
                };
                let Some(wins) = &spell_info.wins else {
                    panic!()
                };
                let winrate = format!("{}%", wins / games);
                let mut spells = SummonerSpellInfo::new(winrate);
                match self.data_dragon.get_version().await {
                    Ok(version) => {
                        let spell_json = self.data_dragon.summoners_json(&version).await;
                        match spell_json {
                            Ok(json) => {
                                for (_, data) in json.data.iter() {
                                    let Some(spell_array) = &spell_info.spells else {
                                        panic!()
                                    };
                                    let spell_one = spell_array[0].to_string();
                                    let spell_two = spell_array[1].to_string();
                                    if spell_one == data.key {
                                        spells.spell_one = Spell {
                                            name: data.name.clone(),
                                            description: data.description.clone(),
                                            url: format!("http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}", version, data.image.full),
                                            local_image: format!("{}.png", data.name),
                                        }
                                    } else if spell_two == data.key {
                                        spells.spell_two = Spell {
                                            name: data.name.clone(),
                                            description: data.description.clone(),
                                            url: format!("http://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}", version, data.image.full),
                                            local_image: format!("{}.png", data.name),
                                        }
                                    }
                                }
                                Ok(spells)
                            }
                            Err(err) => {
                                if err.is_connection() {
                                    Err(ErrorMap::DataDragonErrors(err))
                                } else {
                                    self.community_dragon_summoners(spell_info, spells).await
                                }
                            }
                        }
                    }
                    Err(err) => {
                        if err.is_connection() {
                            Err(ErrorMap::DataDragonErrors(err))
                        } else {
                            self.community_dragon_summoners(spell_info, spells).await
                        }
                    }
                }
            }
            Err(err) => Err(err.to_owned()),
        }
    }

    async fn community_dragon_summoners(
        &self,
        spell_info: &SummonerSpells,
        mut spells: SummonerSpellInfo,
    ) -> Result<SummonerSpellInfo, ErrorMap> {
        let Some(spell_array) = &spell_info.spells else {
            panic!()
        };
        let community_dragon = CommunityDragon::new(self.lang, self.client);
        let summoner_spell_json = community_dragon.summoner_spells().await;
        match summoner_spell_json {
            Ok(json) => {
                for spell_iter in json {
                    let url = |spell_path: String| {
                        let base_url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default";
                        if let Some(item_path_pos) = spell_path.find("/DATA/") {
                            let split = spell_path.split_at(item_path_pos);
                            let url = format!("{}{}", base_url, split.1);
                            url.to_lowercase()
                        } else {
                            unreachable!();
                        }
                    };

                    if spell_iter.id == spell_array[0] {
                        spells.spell_one = Spell {
                            name: spell_iter.name.clone(),
                            description: spell_iter.description.clone(),
                            url: url(spell_iter.icon_path),
                            local_image: format!("{}.png", spell_iter.name),
                        };
                    } else if spell_iter.id == spell_array[1] {
                        spells.spell_two = Spell {
                            name: spell_iter.name.clone(),
                            description: spell_iter.description.clone(),
                            url: url(spell_iter.icon_path),
                            local_image: format!("{}.png", spell_iter.name),
                        };
                    }
                }
                Ok(spells)
            }
            Err(err) => Err(ErrorMap::CommunityDragonErrors(err)),
        }
    }
}
