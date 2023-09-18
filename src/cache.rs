use std::collections::HashMap;
use memprocfs::VmmProcess;
use crate::constants::offsets::*;
use crate::data::*;
use crate::function::*;
use crate::mem::*;

pub enum Cache {
    High, // near position
    Medium, // far position
    Low, // dead player
    Static, // pointer
}


#[derive(Debug, Clone, Default)]
pub struct CacheData {
    cache_high: HashMap<u64, Player>,
    cache_medium: HashMap<u64, Player>,
    cache_low: HashMap<u64, Player>,
    cache_static: HashMap<u64, Player>,
}
pub fn initialize_match(vp: VmmProcess, base: u64) {
    let mut cache_data = CacheData::default();
    let player_pointer = get_player_pointer(vp, base + CL_ENTITYLIST);
    let mut local_player = LocalPlayer { base, ..Default::default() };
    for pointer in player_pointer {
        let player = Player { pointer, ..Default::default() };
        match_player_cache(vp, player , &mut cache_data)

    }
}
/// addr -> player pointer
fn match_player_cache(vp: VmmProcess, player: Player, cache_data: &mut CacheData) {
    if player.status.dead > 0 { cache_data.cache_low.insert(player.pointer, player); }
    else if player.status.knocked > 0 { cache_data.cache_medium.insert(player.pointer, player); }
    else if player.status.health > 0 { cache_data.cache_medium.insert(player.pointer, player); }
}