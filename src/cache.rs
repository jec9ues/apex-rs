use std::collections::HashMap;
use egui_backend::egui::*;
use egui_backend::egui::epaint::PathShape;
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
            if data.status.dead > 0 {continue};
            self.local_player.camera_position = Pos3::from_array(read_f32_vec(vp, data.pointer + CAMERA_POSITION, 3).as_slice().try_into().unwrap());
            self.local_player.update_position(vp);
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
        }
        println!("{}", self.cache_pointer.cache_high.len());
    }

    pub fn update_cache_high(&mut self, vp: VmmProcess) {
        for pointer in &mut self.cache_pointer.cache_high {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                // player.status.update(vp, &player.pointer);
                player.update_position(vp);
                player.update_distance(vp, &self.cache_data.local_player.position);
                player.update_bone_position(vp);
            }
        }
    }
    pub fn update_cache_medium(&mut self, vp: VmmProcess) {
        for pointer in &mut self.cache_pointer.cache_medium {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                // player.status.update(vp, &player.pointer);
                player.update_position(vp);
                player.update_distance(vp, &self.cache_data.local_player.position);
                player.update_bone_position(vp);
            }
        }
    }
    pub fn update_cache_low(&mut self, vp: VmmProcess) {
        for pointer in &mut self.cache_pointer.cache_low {
            if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                // player.status.update(vp, &player.pointer);
                player.update_position(vp);
                player.update_distance(vp, &self.cache_data.local_player.position);
                player.update_bone_position(vp);
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

    }

    /*    pub fn draw_bones_width(&mut self, ptr: Painter) {
            for pointer in &mut self.cache_pointer.cache_high {
                if let Some(player) = self.cache_data.players.get_mut(&pointer) {
                    let mut draw: Vec<Pos2> = Vec::new();
                    let mut bones = [
                        &mut player.hitbox.head,
                        &mut player.hitbox.neck,
                        &mut player.hitbox.upper_chest,
                        &mut player.hitbox.lower_chest,
                        &mut player.hitbox.stomach,
                        &mut player.hitbox.hip,
                        &mut player.hitbox.left_shoulder,
                        &mut player.hitbox.left_elbow,
                        &mut player.hitbox.left_hand,
                        &mut player.hitbox.right_shoulder,
                        &mut player.hitbox.right_elbow,
                        &mut player.hitbox.right_hand,
                        &mut player.hitbox.left_thigh,
                        &mut player.hitbox.left_knee,
                        &mut player.hitbox.left_foot,
                        &mut player.hitbox.right_thigh,
                        &mut player.hitbox.right_knee,
                        &mut player.hitbox.right_foot,
                    ];

                    for bone in bones.iter_mut() {
                        bone.position_2d = world_to_screen(self.cache_data.local_player.view_matrix, bone.position, Pos2 {x: 2560.0, y: 1440.0});
                        bone.width = 10.0;
                        bone.left = Pos2 {x: bone.position_2d.x - bone.width / 2.0, y: bone.position_2d.y};
                        bone.right = Pos2 {x: bone.position_2d.x + bone.width / 2.0, y: bone.position_2d.y};
                    };

                    draw.push(player.hitbox.head.left);
                    draw.push(player.hitbox.neck.left);
                    draw.push(player.hitbox.left_shoulder.left);
                    draw.push(player.hitbox.left_elbow.left);
                    draw.push(player.hitbox.left_hand.left);
                    draw.push(player.hitbox.left_hand.right);
                    draw.push(player.hitbox.left_elbow.right);
                    draw.push(player.hitbox.left_shoulder.right);
                    draw.push(player.hitbox.upper_chest.left);
                    draw.push(player.hitbox.lower_chest.left);
                    draw.push(player.hitbox.hip.left);
                    draw.push(player.hitbox.left_thigh.left);
                    draw.push(player.hitbox.left_knee.left);
                    draw.push(player.hitbox.left_foot.left);
                    draw.push(player.hitbox.left_foot.right);
                    draw.push(player.hitbox.left_knee.right);
                    draw.push(player.hitbox.left_thigh.right);
                    draw.push(player.hitbox.right_thigh.left);
                    draw.push(player.hitbox.right_knee.left);
                    draw.push(player.hitbox.right_foot.left);
                    draw.push(player.hitbox.right_foot.right);
                    draw.push(player.hitbox.right_knee.right);
                    draw.push(player.hitbox.right_thigh.right);
                    draw.push(player.hitbox.hip.right);
                    draw.push(player.hitbox.lower_chest.right);
                    draw.push(player.hitbox.upper_chest.right);
                    draw.push(player.hitbox.right_shoulder.left);
                    draw.push(player.hitbox.right_elbow.left);
                    draw.push(player.hitbox.right_hand.left);
                    draw.push(player.hitbox.right_hand.right);
                    draw.push(player.hitbox.right_elbow.right);
                    draw.push(player.hitbox.right_shoulder.right);
                    draw.push(player.hitbox.neck.right);
                    draw.push(player.hitbox.head.right);
                    draw.push(player.hitbox.head.left);
                    ptr.add(Shape::Path(PathShape::closed_line(draw, Stroke::new(1.0, Color32::RED))));

                }
            }
        }*/
}