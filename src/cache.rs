use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use memprocfs::{CONFIG_OPT_REFRESH_ALL, Vmm, VmmProcess, VmmProcessInfo};
use serde::{Deserialize, Serialize};
use crate::config::Config;
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


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CachePtr {
    pub cache_high: Vec<u64>,
    pub cache_medium: Vec<u64>,
    pub cache_low: Vec<u64>,
    pub cache_static: Vec<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheData {
    pub local_player: LocalPlayer,
    pub target: Player,
    pub players: HashMap<u64, Player>,
}
impl CacheData {
/*    pub fn get_players_bones_position(&mut self, vp: VmmProcess) -> Vec<Pos3> {
        let mut res: Vec<Pos3> = Vec::new();
        for (pointer, mut data) in &mut self.players {
            // data.update_bone_position(vp);
            if data.status.dead > 0 {continue};
            self.local_player.camera_position = Pos3::from_array(read_f32_vec(vp, data.pointer + CAMERA_POSITION, 3).as_slice().try_into().unwrap());
            self.local_player.update_position(vp);
            res.extend(data.get_bones_position());
        };
        res
    }*/
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessData {
    pub pid: u32,
    pub base: u64,
    pub name: String,
}

/*impl ProcessData {
    pub fn init(process_name: &str) -> Result<&VmmProcess, Box<dyn std::error::Error>>{
        println!("DMA for Apex - START");
        println!("========================================");
        println!("get vmm path");
        let mut path = match env::current_dir() {
            Ok(mut current_dir) => {
                current_dir.push("vmm.dll");
                println!("vmm path -> {:?}", current_dir);
                current_dir
            }
            Err(err) => {
                panic!("Error -> {:?}", err);
            }
        };
        let path = path.to_str()?;
        // println!("{:?}", path);
        println!("========================================");
        let vmm_args = ["-device", "fpga", "-memmap", "auto"].to_vec();
        println!("init vmm dll");
        let vmm = Vmm::new(path, &vmm_args)?;
        println!("========================================");
        println!("fresh vmm memory");
        let _ = vmm.set_config(CONFIG_OPT_REFRESH_ALL, 1);
        println!("========================================");
        println!("get vmmprocess from name");
        let vp = vmm.process_from_name(process_name)?;
        println!("{process_name} pid -> {}", vp.pid);
        Ok(&vp)

    }
}*/

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub base: u64,
    // pub proc: ProcessData,
    pub cache_pointer: CachePtr,
    pub cache_data: CacheData,
    pub grenade: [f32; 2],
    pub key: KeyData,
    pub config: Config,
    // pub table: DataTable,
}

/*#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub base: u64,
    // pub proc: ProcessData,
    pub cache_pointer: CachePtr,
    pub cache_data: CacheData,
    pub key: KeyData,
    pub config: Config,
    // pub table: DataTable,
}*/

impl Data {
    //TODO: improve get player by distance, health, fov

    pub fn get_near_player(&self) -> Player {
        let mut near_player: &Player = &Default::default();
        let mut last_distance: f32 = 0.0;
        for pointer in &self.cache_pointer.cache_high {
            if let Some(player) = self.cache_data.players.get(&pointer) {
                if player.distance == 0.0 { continue };
                println!("distance -> {}", player.distance);
                if last_distance < player.distance && player.status.team != self.cache_data.local_player.status.team && player.status.dead == 0{
                    near_player = player;
                }
            }
        }
        near_player.clone()
    }
    pub fn get_near_crosshair_player(&self) -> Player {
        let mut near_player: &Player = &Default::default();
        let mut last_distance: f32 = 999.0;
        let mut last_dis: f32 = 999.0;
        for pointer in &self.cache_pointer.cache_high {
            if let Some(player) = self.cache_data.players.get(&pointer) {
                // println!("ptr -> {:?}", player.pointer);
                // println!("head pos -> {:?}", player.hitbox.head.position_2d);
                // println!("distance -> {}", player.distance);
                if player.position_2d == Pos2::ZERO || player.distance > 150.0 || player.status.dead > 0 || player.status.knocked > 0 { continue }
                // if player.distance > 150.0 { continue }
                let dis = distance2d(&self.config.screen.center, &player.position_2d);
                if last_distance > dis{
                    last_distance = dis;
                    last_dis = player.distance;
                    near_player = player;
                }
            }
        }
        // println!("distance -> {}", near_player.status.platform_id);
        near_player.clone()
    }

    pub fn get_near_crosshair_target(&self, distance: f32, team_check: bool) -> Player {
        let mut near_player: &Player = &Default::default();
        let mut last_distance: f32 = 999.0;
        for pointer in &self.cache_pointer.cache_medium {
            if let Some(player) = self.cache_data.players.get(&pointer) {

                if team_check {
                    if player.status.team == self.cache_data.local_player.status.team { continue }
                }

                if player.position_2d == Pos2::ZERO || player.distance > distance || player.status.dead > 0 || player.status.knocked > 0 { continue }

                let dis = distance2d(&self.config.screen.center(), &player.position_2d);
                if last_distance > dis{
                    last_distance = dis;
                    near_player = player;
                }
            }
        }
        // println!("distance -> {}", near_player.status.platform_id);
        near_player.clone()
    }
    pub fn initialize(&mut self, vp: VmmProcess, base: u64) {
        //init data
        self.base = base;

        // init local player
        self.cache_data.local_player.update_pointer(vp, self.base);
        self.cache_data.local_player.update_position(vp);
        self.cache_data.local_player.update_bone_index(vp);
        self.cache_data.local_player.update_bone_position(vp);
        self.cache_data.local_player.status.initialize(vp, self.cache_data.local_player.pointer, self.base, 1);


        // init other player
        for pointer in get_player_pointer_index(vp, base + CL_ENTITYLIST) {
            let mut player = Player { index: pointer[0], pointer: pointer[1], ..Default::default() };
            player.status.initialize(vp, player.pointer, self.base, player.index);
            if player.status.team_index == self.cache_data.local_player.status.team_index && player.status.team == self.cache_data.local_player.status.team {
                continue
                // pass local player
            }
/*            if player.status.team == self.cache_data.local_player.status.team {
                continue
            };*/
            player.update_pointer(vp);
            player.update_bone_index(vp);

            player.update_position(vp, self.cache_data.local_player.view_matrix, self.config.screen.size);
            // player.update_bone_position(vp, self.cache_data.local_player.view_matrix, self.config.screen.size);
            player.update_distance(vp, &self.cache_data.local_player.position);
            // println!("distance -> {:?}", player.distance);
            // println!("pos -> {:?} pos -> {:?}", &player.position, &self.cache_data.local_player.position);



            self.cache_pointer.cache_medium.push(player.pointer);
            self.cache_data.players.insert(player.pointer, player);
        }
        // println!("{}", self.cache_pointer.cache_medium.len());
    }
    pub fn update_cache(&mut self, vp: VmmProcess) {
        // local player + players
        let mut now_pointer = get_player_pointer(vp, self.base + CL_ENTITYLIST);
        // println!("now {:?}", now_pointer.len());
        now_pointer.retain(|&x| x != self.cache_data.local_player.pointer);
        // println!("remove now {:?}", now_pointer.len());
        let mut null_pointer_remove: Vec<u64> = Vec::new();
        if now_pointer.len() - 1 != self.cache_pointer.cache_medium.len() {
            let now_list = get_player_pointer_index(vp, self.base + CL_ENTITYLIST);

            for pointer in &self.cache_pointer.cache_medium {
                if now_pointer.contains(pointer) {
                    // remove null pointer
                    null_pointer_remove.push(*pointer);

                }
            }

            self.cache_pointer.cache_medium.retain(|&x| !null_pointer_remove.contains(&x));

            for &pointer_value in &null_pointer_remove {
                self.cache_data.players.remove(&pointer_value);
            }

            for pointer in now_list {

                let mut player = Player { index: pointer[0], pointer: pointer[1], ..Default::default() };

                if player.status.team_index == self.cache_data.local_player.status.team_index && player.status.team == self.cache_data.local_player.status.team {
                    continue
                    // pass local player
                }

                if self.cache_pointer.cache_medium.contains(&player.pointer) {
                    // pass exist pointer
                    continue

                } else {
                    player.update_pointer(vp);
                    player.update_bone_index(vp);

                    // player.update_position(vp, self.cache_data.local_player.view_matrix, self.config.screen.size);
                    player.update_bone_position(vp, self.cache_data.local_player.view_matrix, self.config.screen.size);
                    player.update_distance(vp, &self.cache_data.local_player.position);


                    // push new pointer
                    self.cache_pointer.cache_medium.push(player.pointer);
                    self.cache_data.players.insert(player.pointer, player);
                }
            }
        }

    }
    pub fn update_basic(&mut self, vp: VmmProcess, distance: f32) {
        self.cache_data.local_player.update_position(vp);
        // self.cache_data.local_player.update_bone_position(vp);
        self.cache_data.local_player.update_view_matrix(vp); // 500 µs
        self.cache_data.local_player.update_angle(vp); // 500 µs
        self.key.update_key_state(vp, self.base);

        for pointer in &mut self.cache_pointer.cache_medium {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                if player.distance > distance {
                    continue
                }
                player.update_position(vp, self.cache_data.local_player.view_matrix, self.config.screen.size);
                player.update_distance(vp, &self.cache_data.local_player.position);
            }
        }
    }
    pub fn update_status(&mut self, vp: VmmProcess) {
        // self.cache_data.local_player.status.update(vp, &self.cache_data.local_player.pointer);
        for pointer in &mut self.cache_pointer.cache_medium {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                player.status.update(vp, &player.pointer);
            }
        }
    }
    pub fn update_target(&mut self, vp: VmmProcess, distance: f32, screen_size: Pos2) {
        self.cache_data.target =  self.get_near_crosshair_target(distance, self.config.aim.team_check);

        self.cache_data.target.update_bone_position(vp, self.cache_data.local_player.view_matrix, self.config.screen.size);
        self.cache_data.target.update_bone_position_2d(self.cache_data.local_player.view_matrix, screen_size);
    }

}