use phf::{phf_map, phf_ordered_map};

// These are used in the U.GG JSON to map the value to the human readable name
// This is done for the purpose of code readability, as well as sanity.

/// A list of different roles for LoL to send to the FE
/// this will be changed in the future to send images
/// and use the number/name as a value system
pub static ROLES: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "Top" => "4",
    "Jungle" => "1",
    "Mid" => "5",
    "ADC" => "3",
    "Support" => "2",
};

/// Internal constant for commonly used parts of the UGG JSON files
pub static DATA: phf::Map<&'static str, usize> = phf_map! {
    "perks" => 0,
    "summoner_spells" => 1,
    "starting_items" => 2,
    "mythic_and_core" => 3,
    "abilities" => 4,
    "other_items" => 5,
    "winrate" => 6,
    "false" => 7,
    "shards" => 8,
};

/// Internal constant for commonly used parts of the UGG JSON files
pub static STATS: phf::Map<&'static str, usize> = phf_map! {
    "wins" => 0,
    "matches" => 1,
    "rank" => 2,
    "total_rank" => 3,
    // The stuff inbetween here seems worthless!
    "bans" => 10,
    "total_matches" => 11,
    "matchups" => 12, /* 2D Array, [0] = champion_id, [1] = loses, [2] = matches */
    "real_matches" => 13,
    "stdevs" => 14,
    "effective_winrate" => 15,
    "distribution_count" => 16,
    "distribution_mean" => 17,
    "distribution_stdevs" => 18,
    "be_all_picks" => 19,
};
