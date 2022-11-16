use phf::{phf_map, phf_ordered_map};

// These are used in the U.GG JSON to map the value to the human readable name
// This is done for the purpose of code readability, as well as sanity.
pub static REGIONS: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "World" => "12",
    "North America" => "1",
    "EU West" => "2",
    "EU North" => "4",
    "Korea" => "3",
    "Brazil" => "5",
    "LA North" => "6",
    "LA South" => "7",
    "OCE" => "8",
    "Russia" => "9",
    "Turkey" => "10",
    "Japan" => "11",
};

pub static TIERS: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "Challenger" => "1",
    "Grandmaster" => "13",
    "Master" => "2",
    "Diamond" => "3",
    "Platinum" => "4",
    "Gold" => "5",
    "Silver" => "6",
    "Bronze" => "7",
    "Iron" => "15",
    "Overall" => "8",
    "Master Plus" => "14",
    "Diamond Plus" => "11",
    "Diamond 2 Plus" => "12",
    "Platinum Plus" => "10",
};

pub static ROLES: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "Top" => "4",
    "Jungle" => "1",
    "Mid" => "5",
    "ADC" => "3",
    "Support" => "2",
};

pub static DATA: phf::Map<&'static str, usize> = phf_map! {
    "perks" => 0,
    "summoner_spells" => 1,
    "starting_items" => 2,
    "mythic_and_core" => 3,
    "abilities" => 4,
    "other_items" => 5,
    "shards" => 8,
};

pub static STATS: phf::Map<&'static str, usize> = phf_map! {
    "wins" => 0,
    "matches" => 1,
    "rank" => 2,
    "total_rank" => 3,
    "bans" => 10,
    "total_matches" => 11,
    "real_matches" => 13,
};