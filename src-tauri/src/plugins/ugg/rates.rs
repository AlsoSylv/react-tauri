use super::{structs, requests::ranking, constants::STATS};

impl structs::Data {
        //The format is used here to get an exact result from the floating point math
        pub async fn winrate(&self) -> Result<String, i64> {
            let request = ranking(
                self.name.clone(),
                self.role.clone(),
                self.rank.clone(),
                self.region.clone()).await;
            match request {
                Ok(json) => {
                    let Some(matches) = json[STATS["matches"]].as_f64() else {
                        return Err(206);
                    };
    
                    let Some(wins) = json[STATS["wins"]].as_f64() else {
                        return Err(205);
                    };
    
                    let win_rate = wins / matches;
                    Ok(format!("{:.1$}%", win_rate * 100.0, 1))
                }
                Err(err) => Err(err)
            }
        }
        
        pub async fn ban_rate(&self) -> Result<String, i64> {
            let request = ranking(
                self.name.clone(),
                self.role.clone(),
                self.rank.clone(),
                self.region.clone()).await;
            match request {
                Ok(json) => {
                    let Some(matches) = json[STATS["total_matches"]].as_f64() else {
                        return Err(206);
                    };
    
                    let Some(bans)= json[STATS["bans"]].as_f64() else {
                        return Err(205);
                    };
                    let ban_rate = bans / matches;
                    Ok(format!("{:.1$}%", ban_rate * 100.0, 1))
                }
                Err(err) => Err(err)
            }
        }
    
        pub async fn pick_rate(&self) -> Result<String, i64> {
            let request = ranking (
                self.name.clone(),
                self.role.clone(),
                self.rank.clone(),
                self.region.clone()).await;    
            match request {
                Ok(json) => {
                    let Some(matches) = json[STATS["total_matches"]].as_f64() else {
                        return Err(206);
                    };
    
                    let Some(picks) = json[STATS["matches"]].as_f64() else {
                        return Err(205);
                    };
    
                    let pick_rate = picks / matches;
                    Ok(format!("{:.1$}%", pick_rate * 100.0, 1))
                }
                Err(err) => Err(err)
            }
        }
}