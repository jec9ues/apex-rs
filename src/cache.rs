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
pub struct CachePtr {
    pub cache_high: Vec<u64>,
    pub cache_medium: Vec<u64>,
    pub cache_low: Vec<u64>,
    pub cache_static: Vec<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct CacheData {
    pub players: HashMap<u64, Player>,
}
#[derive(Debug, Clone, Default)]
pub struct Data {
    pub cache_pointer: CachePtr,
    pub cache_data: CacheData,
}

impl CacheData {
    pub fn get_players_bones_position(&mut self, vp: VmmProcess) -> Vec<Pos3> {
        let mut res: Vec<Pos3> = Vec::new();
        for (pointer, mut data) in &mut self.players {
            data.update_bone_position(vp);
            res.extend(data.get_bones_position());
        };
        res
    }



}
pub fn initialize_match(vp: VmmProcess, base: u64) -> Data {
    let mut data = Data::default();
    let player_pointer = get_player_pointer_index(vp, base + CL_ENTITYLIST);
    for pointer in &player_pointer {
        let mut player = Player { index: pointer[0], pointer: pointer[1], ..Default::default() };
        player.update_pointer(vp);
        player.update_bone_index(vp);
        player.update_bone_position(vp);
        data.cache_pointer.cache_high.push(pointer[1]);
        data.cache_data.players.insert(pointer[1], player);
    }
    println!("{:?}", data);
    let mut local_player = LocalPlayer {  ..Default::default() };
    data

}
