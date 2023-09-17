use std::collections::HashMap;
use crate::data::Player;

enum Cache {
    High, // near position
    Medium, // far position
    Low, // dead player
    Static, // pointer
}

pub fn cache_manage() {
    let mut cache_high: HashMap<u64, Player> = HashMap::new();
    let mut cache_medium: HashMap<u64, Player> = HashMap::new();
    let mut cache_low: HashMap<u64, Player> = HashMap::new();
    let mut cache_static: HashMap<u64, Player> = HashMap::new();
}


pub fn initialize_match() {

}
fn assign_to_cache(player: Player) {
    let cache_level = match player.status {
        Status::Low => CacheLevel::Level1,
        Status::Medium => CacheLevel::Level2,
        Status::High => CacheLevel::Level3,
    };

    match cache_level {
        CacheLevel::Level1 => cache_level1.insert(player.id, player),
        CacheLevel::Level2 => cache_level2.insert(player.id, player),
        CacheLevel::Level3 => cache_level3.insert(player.id, player),
    };
}