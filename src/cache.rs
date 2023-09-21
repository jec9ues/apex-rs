use std::collections::{HashMap, HashSet};
use memprocfs::VmmProcess;
use crate::constants::offsets::*;
use crate::data::*;
use crate::function::*;
use crate::mem::*;
use crate::math::*;

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
    pub local_player: LocalPlayer,
    pub players: HashMap<u64, Player>,
}
impl CacheData {
    pub fn get_players_bones_position(&mut self, vp: VmmProcess) -> Vec<Pos3> {
        let mut res: Vec<Pos3> = Vec::new();
        for (pointer, mut data) in &mut self.players {
            // data.update_bone_position(vp);
            res.extend(data.get_bones_position());
        };
        res
    }
}

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub base: u64,
    pub cache_pointer: CachePtr,
    pub cache_data: CacheData,
}

impl Data {
    pub fn initialize(&mut self, vp: VmmProcess, base: u64) {
        //init data
        self.base = base;

        // init local player
        self.cache_data.local_player.update_pointer(vp, self.base);
        self.cache_data.local_player.status.initialize(vp, self.cache_data.local_player.pointer, self.base, 1);
        self.cache_data.local_player.update_position(vp);
        // init other player
        for pointer in get_player_pointer_index(vp, base + CL_ENTITYLIST) {
            let mut player = Player { index: pointer[0], pointer: pointer[1], ..Default::default() };
            player.status.initialize(vp, pointer[1], self.base, pointer[0]);
            if player.status.team == self.cache_data.local_player.status.team {
                continue
            };
            player.update_pointer(vp);
            player.update_bone_index(vp);

            player.update_position(vp);
            player.update_distance(vp, &self.cache_data.local_player.position);
            player.update_bone_position(vp);
            // println!("distance -> {:?}", player.distance);
            // println!("pos -> {:?} pos -> {:?}", &player.position, &self.cache_data.local_player.position);


/*            if player.distance > 50.0 {
                self.cache_pointer.cache_low.push(pointer[1]);
            } else if player.status.dead > 0 {
                self.cache_pointer.cache_low.push(pointer[1]);
            } else if player.status.knocked > 0 {
                self.cache_pointer.cache_medium.push(pointer[1]);
            } else {
                self.cache_pointer.cache_high.push(pointer[1]);
            };*/
            self.cache_pointer.cache_high.push(pointer[1]);
            self.cache_data.players.insert(pointer[1], player);
            println!("init -> {:?}", self.cache_pointer.cache_high);
        }
    }

    pub fn update_cache_high(&mut self, vp: VmmProcess) {
        for pointer in &mut self.cache_pointer.cache_high {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                player.status.update(vp, &player.pointer);
                player.update_position(vp);
                player.update_distance(vp, &self.cache_data.local_player.position);
                player.update_bone_position(vp);
            }
        }
    }

    pub fn update_cache_medium(&mut self, vp: VmmProcess) {
        for pointer in &mut self.cache_pointer.cache_medium {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                player.status.update(vp, &player.pointer);
                player.update_position(vp);
                player.update_distance(vp, &self.cache_data.local_player.position);
                player.update_bone_position(vp);
            }
        }
    }

    pub fn update_cache_low(&mut self, vp: VmmProcess) {
        for pointer in &mut self.cache_pointer.cache_low {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                player.status.update(vp, &player.pointer);
                player.update_position(vp);
                player.update_distance(vp, &self.cache_data.local_player.position);
                player.update_bone_position(vp);
            }
        }
    }
    pub fn re_cache_pointer(&mut self, vp: VmmProcess) {
        let mut high_remove = Vec::new(); // 用于存储需要删除的元素
        let mut medium_remove = Vec::new(); // 用于存储需要删除的元素
        let mut low_remove = Vec::new(); // 用于存储需要删除的元素
        let mut items_to_remove = HashSet::new();

        for pointer in &mut self.cache_pointer.cache_high {
            if let Some(player) = self.cache_data.players.get(&pointer) {
                if player.distance > 50.0 {
                    self.cache_pointer.cache_low.push(player.pointer);
                    high_remove.push(player.pointer);
                }
                else if player.status.dead > 0 {
                    self.cache_pointer.cache_low.push(player.pointer);
                    high_remove.push(player.pointer);
                } else if player.status.knocked > 0 {
                    self.cache_pointer.cache_medium.push(player.pointer);
                    high_remove.push(player.pointer);
                }
            }
        }

        for pointer in &mut self.cache_pointer.cache_medium {
            if let Some(player) = self.cache_data.players.get(&pointer) {
                if player.distance < 50.0 && player.status.knocked == 0{
                    self.cache_pointer.cache_high.push(player.pointer);
                    medium_remove.push(player.pointer);
                }
            }
        }

        for pointer in &mut self.cache_pointer.cache_low {
            if let Some(player) = self.cache_data.players.get(&pointer) {
                if player.distance < 50.0 && player.status.dead == 0{
                    self.cache_pointer.cache_high.push(player.pointer);
                    low_remove.push(player.pointer);
                }
            }
        }
        for item in high_remove.iter().chain(medium_remove.iter()).chain(low_remove.iter()) {
            items_to_remove.insert(*item);
        }
        self.cache_pointer.cache_high.retain(|&x| !items_to_remove.contains(&x));
        self.cache_pointer.cache_medium.retain(|&x| !items_to_remove.contains(&x));
        self.cache_pointer.cache_low.retain(|&x| !items_to_remove.contains(&x));

    }
}