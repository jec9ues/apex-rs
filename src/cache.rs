use std::collections::HashMap;
use std::sync::Condvar;
use egui_backend::egui::*;
use egui_backend::egui::epaint::PathShape;
use egui_window_glfw_passthrough::glfw::WindowEvent::Pos;
use memprocfs::VmmProcess;
use crate::config::Config;
use crate::constants::offsets::*;
use crate::data::*;
use crate::function::*;
use crate::mem::*;
use crate::math::*;
use crate::menu::dbg_player;

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

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub base: u64,
    pub cache_pointer: CachePtr,
    pub cache_data: CacheData,
    pub grenade: [f32; 2],
    pub key: KeyData,
    pub config: Config,
    // pub table: DataTable,
}

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
                // self.cache_data.players.remove(&pointer_value);
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
/*    pub fn update_cache_high(&mut self, vp: VmmProcess) {

        self.cache_data.local_player.status.update(vp, &self.cache_data.local_player.pointer);
        self.cache_data.local_player.update_bone_position(vp);
        self.cache_data.local_player.update_view_matrix(vp); // 500 µs
        self.cache_data.local_player.update_angle(vp); // 500 µs


        self.key.update_key_state(vp, self.base);
        for pointer in &mut self.cache_pointer.cache_high {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                // player.status.update(vp, &player.pointer);
                // player.status.update(vp, &player.pointer);

                player.update_position(vp, self.cache_data.local_player.view_matrix);
                player.update_distance(vp, &self.cache_data.local_player.position);
                // if player.distance > 50.0 {continue}
                // player.update_bone_index(vp);
                // player.update_bone_position(vp);

                self.cache_data.target.update_bone_position_2d(self.cache_data.local_player.view_matrix);
            }
        }
        self.cache_data.target =  self.get_near_crosshair_player();
    }
    pub fn update_cache_medium(&mut self, vp: VmmProcess) {
        for pointer in &mut self.cache_pointer.cache_medium {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                // player.status.update(vp, &player.pointer);
                player.update_position(vp, self.cache_data.local_player.view_matrix);
                player.update_distance(vp, &self.cache_data.local_player.position);
                // player.update_bone_index(vp);
                player.update_bone_position(vp);
                player.status.update(vp, &player.pointer);

                self.cache_data.target.update_bone_position_2d(self.cache_data.local_player.view_matrix);
            }
        }
    }
    pub fn update_cache_low(&mut self, vp: VmmProcess) {
        for pointer in &mut self.cache_pointer.cache_low {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                // player.status.update(vp, &player.pointer);
                player.update_position(vp, self.cache_data.local_player.view_matrix);
                player.update_distance(vp, &self.cache_data.local_player.position);
                // player.update_bone_index(vp);
                player.update_bone_position(vp);
                player.status.update(vp, &player.pointer);

                self.cache_data.target.update_bone_position_2d(self.cache_data.local_player.view_matrix);
            }
        }
    }
    pub fn re_cache_pointer(&mut self, vp: VmmProcess) {
        let mut high_remove = Vec::new();
        let mut medium_remove = Vec::new();
        let mut low_remove = Vec::new();
        for pointer in &mut self.cache_pointer.cache_high {
            if let Some(player) = self.cache_data.players.get(&pointer) {
                if player.distance > 100.0 {
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
                if player.distance < 100.0 && player.status.knocked == 0{
                    self.cache_pointer.cache_high.push(player.pointer);
                    medium_remove.push(player.pointer);
                }
            }
        }

        for pointer in &mut self.cache_pointer.cache_low {
            if let Some(player) = self.cache_data.players.get(&pointer) {
                // println!("distance -> {} dead -> {}", player.distance, player.status.dead);
                if player.distance < 100.0 && player.status.dead == 0{
                    self.cache_pointer.cache_high.push(player.pointer);
                    low_remove.push(player.pointer);
                }
            }
        }
        for item in high_remove {
            self.cache_pointer.cache_high.retain(|&x| x != item);
        }
        for item in medium_remove {
            self.cache_pointer.cache_medium.retain(|&x| x != item);
        }

        for item in low_remove {
            self.cache_pointer.cache_low.retain(|&x| x != item);
        }

    }*/
    pub fn dbg_view(&self, ui: &mut Ui) {
        for i in &self.cache_data.players {

            dbg_player(i.1, ui);
        }
    }

}