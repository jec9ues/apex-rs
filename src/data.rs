use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, format, Formatter};
use egui_backend::egui::*;
use log::*;
use memprocfs::*;
use pretty_hex::PrettyHex;
use crate::constants::offsets::*;
use crate::function::*;
use crate::math::*;
use crate::mem::*;
use named_constants::named_constants;
use serde::{Deserialize, Serialize};
use crate::convert_coordinates;

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub index: u64,
    pub pointer: u64,
    pub bone_pointer: u64,
    pub hitbox: Hitbox,
    pub status: Status,
    pub position: Pos3,
    pub position_2d: Pos2,
    pub distance: f32,
    pub distance_2d: f32,
    pub rate: f32,
    pub error: DataError,
}


#[derive(Debug, Copy, Clone, Default)]
pub struct Hitbox {
    pub head: Bone,
    pub neck: Bone,
    pub upper_chest: Bone,
    pub lower_chest: Bone,
    pub stomach: Bone,
    pub hip: Bone,
    pub left_shoulder: Bone,
    pub left_elbow: Bone,
    pub left_hand: Bone,
    pub right_shoulder: Bone,
    pub right_elbow: Bone,
    pub right_hand: Bone,
    pub left_thigh: Bone,
    pub left_knee: Bone,
    pub left_foot: Bone,
    pub right_thigh: Bone,
    pub right_knee: Bone,
    pub right_foot: Bone,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Bone {
    pub index: usize,
    pub position: Pos3,
    pub position_2d: Pos2,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Pos3 {
    pub x: f32,

    pub y: f32,

    pub z: f32,
}

impl Pos3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub fn from_array(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
    pub fn to_pos2(&self) -> Pos2 {
        Pos2 {
            x: self.x,
            y: self.y,

        }
    }
    pub fn add(&self, v: &Pos3) -> Pos3 {
        Pos3 { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }

    }
    pub fn sub(&self, v: &Pos3) -> Pos3 {
        Pos3 { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }
    }
    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }
    pub fn len2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn muls(&self, s: f32) -> Pos3 {
        Pos3 { x: self.x * s, y: self.y * s, z: self.z * s }
    }
    pub fn qangle(&self) -> Pos3 {
        let tmp;
        let yaw;
        let pitch;

        if self.y == 0.0 && self.x == 0.0 {
            yaw = 0.0;
            if self.z > 0.0 {
                pitch = 270.0;
            }
            else {
                pitch = 90.0;
            }
        }
        else {
            yaw = f32::atan2(self.y, self.x).to_degrees();
            tmp = (self.x * self.x + self.y * self.y).sqrt();
            pitch = f32::atan2(-self.z, tmp).to_degrees();
        }
        Pos3 { x: pitch, y: yaw, z: 0.0 }
    }

    pub fn qvec(&self) -> Pos3 {
        let pitch = self.x.to_radians();
        let (sp, cp) = pitch.sin_cos();
        let yaw = self.y.to_radians();
        let (sy, cy) = yaw.sin_cos();
        Pos3 { x: cp * cy, y: cp * sy, z: -sp }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct WeaponX {
    pub weapon_handle: u64,
    pub weapon_entity: u64,
    pub index: u16,
    pub projectile_speed: f32,
    pub projectile_scale: f32,
    pub zoom_fov: f32,
    pub ammo: u16,
    pub semi_auto: u16,
    pub selected_slot: u8,
}
impl WeaponX {
    /// addr -> local_player address
    pub fn update(&mut self, vp: VmmProcess, addr: u64, base: u64) {

        self.weapon_handle = read_u64(vp, addr + WEAPON);
        self.weapon_handle &= 0xffff;
        self.weapon_entity = read_u64(vp, base + CL_ENTITYLIST + (self.weapon_handle << 5));
        self.index = read_u16(vp, self.weapon_entity + WEAPON_NAME);
        self.projectile_speed = read_f32(vp, self.weapon_entity + BULLET_SPEED);
        self.projectile_scale = read_f32(vp, self.weapon_entity + BULLET_SCALE);
        self.zoom_fov = read_f32(vp, self.weapon_entity + ZOOM_FOV);
        self.ammo = read_u16(vp, self.weapon_entity + AMMO);
        self.semi_auto = read_u16(vp, self.weapon_entity + SEMI_AUTO);
        self.selected_slot = read_u8(vp, addr + SELECTED_SLOT);

        println!("wephandle -> {:x}", self.weapon_handle);
        println!("wep_entity -> {:x}", self.weapon_entity);
        println!("index -> {:?}", self.index);
        println!("projectile_speed -> {:?}", self.projectile_speed);
        println!("projectile_scale -> {:?}", self.projectile_scale);
        println!("zoom_fov -> {:?}", self.zoom_fov);
        println!("ammo -> {:?}", self.ammo);
        println!("semi_auto -> {:?}", self.semi_auto);
        println!("selected_slot -> {:?}", self.selected_slot);
        // let raw = read_mem(vp, self.weapon_entity + BITFIELD_FROM_PLAYER, 0x10);
        // let b1 = read_u16(vp, self.weapon_entity + BITFIELD_FROM_PLAYER);
        // let b2 = read_u16(vp, self.weapon_entity + BITFIELD_INTERNAL);
        // let b3 = read_u16(vp, self.weapon_entity + BITFIELD_CURRENT);
        // let b4 = read_u16(vp, self.weapon_entity + BITFIELD_DISABLED);
        //
        // println!("raw -> {:?}", raw.hex_dump());
        // println!("b1 -> {:?}\n b2 -> {:?}\n b3 -> {:?}\n b4 -> {:?}", b1, b2, b3, b4)
    }
}




#[derive(Debug, Clone, Default, PartialEq)]
pub struct Status {
    pub dead: u16,
    pub knocked: u16,
    pub health: u16,
    pub max_health: u16,
    pub shield: u16,
    pub max_shield: u16,
    pub helmet_type: u16,
    pub armor_type: u16,
    pub last_visible_time: f32,
    pub previous_last_visible_time: f32,
    pub last_crosshair_target_time: f32,
    pub previous_last_crosshair_target_time: f32,
    pub skin: u16,
    pub character: Character,
    pub team: u16,
    pub team_index: u16,
    pub name: String,
    pub platform_id: u64,
}

impl Status {
    /// addr -> entity pointer
    pub fn initialize(&mut self, vp: VmmProcess, addr: u64, base: u64, index: u64) {
        let data = ContinuingData::new(read_mem(vp, addr, 0x4700));
        self.health = data.read_u16(HEALTH); // 0x036c
        self.max_health = data.read_u16(MAX_HEALTH); // 0x04a8

        self.shield = data.read_u16(SHIELD); // 0x01a0
        self.max_shield = data.read_u16(MAX_SHIELD); // 0x01a4

        self.armor_type = data.read_u16(ARMOR_TYPE); // 0x45c4
        self.helmet_type = data.read_u16(HELMET_TYPE); // 0x45c0
        self.skin = data.read_u16(CURRENT_FRAMEMODEL_INDEX); // 0x00d8


        self.team = data.read_u16(TEAM_NUM);
        self.team_index = data.read_u16(TEAM_MEMBER_INDEX);

        self.dead = data.read_u16(LIFE_STATE); // 0x06c8
        self.knocked = data.read_u16(BLEED_OUT_STATE); // 0x26a0

        self.platform_id = data.read_u64(PLATFORM_USER_ID); // 0x2508

        let name_ptr = read_u64(vp, base + NAME_LIST + (index - 1) * 0x10);
        self.name = read_string(vp, name_ptr);
        // println!("squad id -> {}", self.name)
    }

    pub fn update(&mut self, vp: VmmProcess, addr: &u64) {
        let data = ContinuingData::new(read_mem(vp, *addr, 0x4700));
        self.health = data.read_u16(HEALTH); // 0x036c
        self.max_health = data.read_u16(MAX_HEALTH); // 0x04a8

        self.shield = data.read_u16(SHIELD); // 0x01a0
        self.max_shield = data.read_u16(MAX_SHIELD); // 0x01a4

        self.armor_type = data.read_u16(ARMOR_TYPE); // 0x45c4
        self.helmet_type = data.read_u16(HELMET_TYPE); // 0x45c0
        self.skin = data.read_u16(CURRENT_FRAMEMODEL_INDEX); // 0x00d8

        // let player_data_ptr = read_u64(vp, addr + PLAYER_DATA);
        // let player_datas = read_u16(vp, player_data_ptr + LEGENDARY_MODEL_INDEX);
        // let player_data = read_mem(vp, addr + PLAYER_DATA, 0x100);

        self.previous_last_visible_time = self.last_visible_time;
        self.previous_last_crosshair_target_time = self.last_crosshair_target_time;

        self.last_visible_time = data.read_f32(LAST_VISIBLE_TIME); // 0x19B0
        self.last_crosshair_target_time = data.read_f32(LAST_VISIBLE_TIME + 0x8); // 0x19B0

        self.dead = data.read_u16(LIFE_STATE); // 0x06c8
        self.knocked = data.read_u16(BLEED_OUT_STATE); // 0x26a0



        // let mut da = CharacterType::default();
        // da.initialize_character_type();
        // self.character = da.check_character_type(self.skin);

        // let da = read_mem(vp, addr + LAST_VISIBLE_TIME, 0x30);
        // info!("ptr -> {:x} data -> {} direct -> {:?}", player_data_ptr, player_datas, player_data.hex_dump())
        // info!("last visible time -> {}", self.last_visible_time);
        // info!("data -> {:?}", da.hex_dump());
        // println!("test -> {:?}", self)
    }

    pub fn visible(&self) -> bool {
        // (self.last_visible_time - self.previous_last_visible_time) > 0.01
        !(self.last_visible_time == self.previous_last_visible_time)
    }
    pub fn target(&self) -> bool {
        // (self.last_crosshair_target_time - self.previous_last_crosshair_target_time).abs() < 1.0
        !(self.last_crosshair_target_time == self.previous_last_crosshair_target_time)
    }
    pub fn alive(&self) -> bool {
        !(self.dead > 0)
    }
    pub fn knocked(&self) -> bool {
        !(self.knocked > 0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct LocalPlayer {
    pub pointer: u64,
    pub render_pointer: u64,
    pub matrix_pointer: u64,
    pub view_matrix: [[f32; 4]; 4],
    pub status: Status,
    pub position: Pos3,
    pub camera_position: Pos3,
    pub pitch: f32,
    pub yaw: f32,
    pub bone_pointer: u64,
    pub hitbox: Hitbox,
}

impl LocalPlayer {
    pub fn update_pointer(&mut self, vp: VmmProcess, base: u64) {
        self.pointer = read_u64(vp, base + LOCAL_PLAYER);
        self.render_pointer = read_u64(vp, base + VIEW_RENDER);
        self.matrix_pointer = read_u64(vp, self.render_pointer + VIEW_MATRIX);

        self.bone_pointer = read_u64(vp, self.pointer + BONE);
    }

    pub fn update_position(&mut self, vp: VmmProcess) {
        self.position = Pos3::from_array(read_f32_vec(vp, self.pointer + LOCAL_ORIGIN, 3).as_slice().try_into().unwrap());
        self.camera_position = Pos3::from_array(read_f32_vec(vp, self.pointer + CAMERA_POSITION, 3).as_slice().try_into().unwrap());
        // self.camera_angle = read_f32_vec(vp, self.pointer + CAMERA_ANGLES, 2).as_slice().try_into().unwrap();
        // let local: Vec<f32> = read_f32_vec(vp, self.pointer + LOCAL_ORIGIN, 3).as_slice().try_into().unwrap();
        // let vec: Vec<f32> = read_f32_vec(vp, self.pointer + ABS_VECTORORIGIN, 3).as_slice().try_into().unwrap();
        // println!("angles -> {:?}", self.camera_angle);
        // println!("vec -> {:?}", vec);
    }

    pub fn update_view_matrix(&mut self, vp: VmmProcess) {
        self.view_matrix = read_f32_vec(vp, self.matrix_pointer, 16)
            .chunks_exact(4)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<[f32; 4]>>()
            .try_into()
            .unwrap();
    }

    pub fn update_angle(&mut self, vp: VmmProcess) {
        let angle = read_f32_vec(vp, self.pointer + VIEW_ANGLE, 2);
        self.pitch = *angle.get(0).unwrap();
        self.yaw = *angle.get(1).unwrap();
    }
    pub fn set_angle(&mut self, vp: VmmProcess, pitch: f32, yaw: f32) {
        if pitch.is_infinite() || yaw.is_infinite() {
            return;
        }
        let mut angle: Vec<f32> = Vec::new();
        angle.push(pitch);
        angle.push(yaw);
        write_f32_vec(vp, self.pointer + VIEW_ANGLE, angle);
    }
    pub fn set_pitch(&mut self, vp: VmmProcess, pitch: f32) {
        write_f32(vp, self.pointer + VIEW_ANGLE, pitch);
    }
    pub fn set_yaw(&mut self, vp: VmmProcess, yaw: f32) {
        write_f32(vp, self.pointer + VIEW_ANGLE + 0x4, yaw);
    }

    pub fn update_bone_index(&mut self, vp: VmmProcess) {
        let model_pointer = read_u64(vp, self.pointer + STUDIOHDR);
        let studio_hdr = read_u64(vp, model_pointer + 0x8);

        let hitbox_cache = read_u16(vp, studio_hdr + 0x34) as u64;
        let hitbox_array = studio_hdr + ((hitbox_cache & 0xFFFE) << (4 * (hitbox_cache & 1)));

        let index_cache = read_u16(vp, hitbox_array + 0x4);
        let hitbox_index = (index_cache & 0xFFFE) << (4 * (index_cache & 1));
        // 19 is bone amount we need
        let data = read_mem(vp, hitbox_index as u64 + hitbox_array, 20 * 0x20);
        // println!("{:?}", data.hex_dump());
        let bone_index: Vec<u16> = data.chunks_exact(0x20)
            .map(|chunk| u16::from_le_bytes(chunk[..2].try_into().unwrap()))
            .collect();

        // println!("{} -> {:?}", self.pointer, bone_index);
        /*        if bone_index.iter().any(|&x| x > 240) {
                    Err(DataError::BoneError)
                };*/

        if self.status.character == Character::Bloodhound {
            self.hitbox.head.index = bone_index[0] as usize;
            self.hitbox.neck.index = bone_index[1] as usize;
            self.hitbox.upper_chest.index = bone_index[2] as usize;
            self.hitbox.lower_chest.index = bone_index[3] as usize;
            self.hitbox.stomach.index = bone_index[4] as usize;
            self.hitbox.hip.index = bone_index[5] as usize;
            self.hitbox.left_shoulder.index = bone_index[6] as usize;
            self.hitbox.left_elbow.index = bone_index[7] as usize;
            self.hitbox.left_hand.index = bone_index[19] as usize;
            self.hitbox.right_shoulder.index = bone_index[8] as usize;
            self.hitbox.right_elbow.index = bone_index[9] as usize;
            self.hitbox.right_hand.index = bone_index[10] as usize;
            self.hitbox.left_thigh.index = bone_index[11] as usize;
            self.hitbox.left_knee.index = bone_index[12] as usize;
            self.hitbox.left_foot.index = bone_index[13] as usize;
            self.hitbox.right_thigh.index = bone_index[15] as usize;
            self.hitbox.right_knee.index = bone_index[16] as usize;
            self.hitbox.right_foot.index = bone_index[17] as usize;
        } else {
            self.hitbox.head.index = bone_index[0] as usize;
            self.hitbox.neck.index = bone_index[1] as usize;
            self.hitbox.upper_chest.index = bone_index[2] as usize;
            self.hitbox.lower_chest.index = bone_index[3] as usize;
            self.hitbox.stomach.index = bone_index[4] as usize;
            self.hitbox.hip.index = bone_index[5] as usize;
            self.hitbox.left_shoulder.index = bone_index[6] as usize;
            self.hitbox.left_elbow.index = bone_index[7] as usize;
            self.hitbox.left_hand.index = bone_index[8] as usize;
            self.hitbox.right_shoulder.index = bone_index[9] as usize;
            self.hitbox.right_elbow.index = bone_index[10] as usize;
            self.hitbox.right_hand.index = bone_index[11] as usize;
            self.hitbox.left_thigh.index = bone_index[12] as usize;
            self.hitbox.left_knee.index = bone_index[13] as usize;
            self.hitbox.left_foot.index = bone_index[14] as usize;
            self.hitbox.right_thigh.index = bone_index[16] as usize;
            self.hitbox.right_knee.index = bone_index[17] as usize;
            self.hitbox.right_foot.index = bone_index[18] as usize;
        };

        // Ok(())
    }

    pub fn update_bone_position(&mut self, vp: VmmProcess) {
        let vec_abs_origin: [f32; 3] = read_f32_vec(vp, self.pointer + ABS_VECTORORIGIN, 3).as_slice().try_into().unwrap();
        self.position = Pos3::from_array(vec_abs_origin);
        // float: 4 * matrix: 12 * bone: 200
        let data = read_mem(vp, self.bone_pointer, 4 * 12 * 20);
        // println!("{:?}", data.hex_dump());

        let mut f32_num: Vec<f32> = Vec::with_capacity(12 * 20);

        for chunk in data.chunks_exact(4) {
            let mut array: [u8; 4] = [0; 4];
            array.copy_from_slice(chunk);
            let array = f32::from_le_bytes(array);
            f32_num.push(array);
        }

        let matrix: Vec<[[f32; 4]; 3]> = f32_num
            .chunks_exact(4)// [f32; 4]
            .map(|chunk| {
                let mut array: [f32; 4] = [0.0; 4]; // init
                array.copy_from_slice(chunk);
                array
            })
            .collect::<Vec<[f32; 4]>>()
            .chunks_exact(3) // [[f32; 4]; 3]
            .map(|chunk| {
                let mut matrix: [[f32; 4]; 3] = [[0.0; 4]; 3]; // init
                for (i, item) in chunk.iter().enumerate() {
                    matrix[i] = *item;
                }
                matrix
            })
            .collect();

        // println!("name -> {}", self.status.name);
        self.hitbox.head.position = Pos3 {
            x: matrix[self.hitbox.head.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.head.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.head.index][2][3] + vec_abs_origin[2],
        };
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum DataError {
    BoneError,
    #[default]
    None,
}

impl Player {
    pub fn update_pointer(&mut self, vp: VmmProcess) {
        self.bone_pointer = read_u64(vp, self.pointer + BONE);
    }

    /// ptr -> painter
    pub fn get_bones_position(&self) -> Vec<Pos3> {
        let mut res: Vec<Pos3> = Vec::new();
        let bones = [
            &self.hitbox.head.position,
            &self.hitbox.neck.position,
            &self.hitbox.upper_chest.position,
            &self.hitbox.lower_chest.position,
            &self.hitbox.stomach.position,
            &self.hitbox.hip.position,
            &self.hitbox.left_shoulder.position,
            &self.hitbox.left_elbow.position,
            &self.hitbox.left_hand.position,
            &self.hitbox.right_shoulder.position,
            &self.hitbox.right_elbow.position,
            &self.hitbox.right_hand.position,
            &self.hitbox.left_thigh.position,
            &self.hitbox.left_knee.position,
            &self.hitbox.left_foot.position,
            &self.hitbox.right_thigh.position,
            &self.hitbox.right_knee.position,
            &self.hitbox.right_foot.position,
        ];

        for bone in bones.iter() {
            res.push(**bone);
        };
        res
    }

    pub fn position_esp(&self, ptr: Painter) {
        ptr.circle_filled(self.position_2d, 3.0, TEAM_COLOR[self.status.team as usize]);
        ptr.text(self.position_2d,
                 Align2::CENTER_BOTTOM,
                 format!("{:?}", self.status.name),
                 FontId::default(),
                 Color32::LIGHT_BLUE);
        ptr.line_segment(
            [Pos2 { x: self.position_2d.x + 20.0, y: self.position_2d.y + 10.0 },
                Pos2 { x: self.position_2d.x + 20.0, y: self.position_2d.y + 10.0 - self.status.health as f32 / 3.0 }],
            Stroke::new(4.0, Color32::GREEN),
        );

        ptr.line_segment(
            [Pos2 { x: self.position_2d.x + 25.0, y: self.position_2d.y + 10.0 },
                Pos2 { x: self.position_2d.x + 25.0, y: self.position_2d.y + 10.0 - self.status.shield as f32 / 3.0 }],
            Stroke::new(4.0, Color32::BLUE),
        );
    }
    pub fn map_esp(&self, ptr: Painter, pos0: Pos2, rate: [f32; 2]) {
        let player_pos = convert_coordinates(
            self.position.x,
            self.position.y,
            rate[0],
            rate[1],
        );
        ptr.circle(Pos2 { x: pos0.x + player_pos.0, y: pos0.y + player_pos.1 },
                   2.0, Color32::TRANSPARENT, Stroke::new(5.0, Color32::RED));
    }
    pub fn bone_esp(&self, ptr: Painter, distance: f32, color: Color32) {
        if self.status.dead > 0 || self.distance > distance {
            return;
        };
        // println!("bone esp -> {:?}", self.hitbox);
        let mut body: Vec<Pos2> = Vec::new();
        let mut leg: Vec<Pos2> = Vec::new();
        let mut hand: Vec<Pos2> = Vec::new();
        let bones = [
            &self.hitbox.head.position_2d,
            &self.hitbox.neck.position_2d,
            &self.hitbox.upper_chest.position_2d,
            &self.hitbox.lower_chest.position_2d,
            &self.hitbox.stomach.position_2d,
            &self.hitbox.hip.position_2d,
        ];
        let bones2 = [
            &self.hitbox.left_hand.position_2d,
            &self.hitbox.left_elbow.position_2d,
            &self.hitbox.left_shoulder.position_2d,
            &self.hitbox.neck.position_2d,
            &self.hitbox.right_shoulder.position_2d,
            &self.hitbox.right_elbow.position_2d,
            &self.hitbox.right_hand.position_2d,
        ];

        let bones3 = [
            &self.hitbox.left_foot.position_2d,
            &self.hitbox.left_knee.position_2d,
            &self.hitbox.left_thigh.position_2d,
            &self.hitbox.hip.position_2d,
            &self.hitbox.right_thigh.position_2d,
            &self.hitbox.right_knee.position_2d,
            &self.hitbox.right_foot.position_2d,
        ];

        for bone in bones.iter() {
            body.push(**bone);
        };
        for bone in bones2.iter() {
            hand.push(**bone);
        };
        for bone in bones3.iter() {
            leg.push(**bone);
        };


        ptr.add(
            Shape::line(body,
                        Stroke::new(2.0, color)));
        ptr.add(
            Shape::line(hand,
                        Stroke::new(2.0, color)));
        ptr.add(
            Shape::line(leg,
                        Stroke::new(2.0, color)));
        /*        ptr.text(self.hitbox.head.position_2d,
                         Align2::CENTER_BOTTOM,
                         self.status.skin.to_string(),
                         FontId::default(),
                         Color32::WHITE);
                ptr.text(self.hitbox.hip.position_2d,
                         Align2::CENTER_BOTTOM,
                         format!("{:?}", self.status.character),
                         FontId::default(),
                         Color32::LIGHT_RED);*/
        /*        ptr.text(self.hitbox.lower_chest.position_2d,
                         Align2::CENTER_BOTTOM,
                         format!("{:?}", self.status.last_visible_time),
                         FontId::default(),
                         Color32::BLUE);

                ptr.text(self.hitbox.left_elbow.position_2d,
                         Align2::CENTER_BOTTOM,
                         format!("{:?}", self.status.last_crosshair_target_time),
                         FontId::default(),
                         Color32::RED);*/
    }
    pub fn target_line(&self, ptr: Painter, center: Pos2) {
        if self.position_2d == Pos2::ZERO {
            return;
        }
        let nearest_bone = self.get_nearest_bone(center).position_2d;
        ptr.line_segment(
            [nearest_bone, center],
            Stroke::new(2.0, Color32::RED));

        ptr.circle_stroke(nearest_bone, 4.0, Stroke::new(2.0, Color32::GREEN));
    }

    pub fn update_position(&mut self, vp: VmmProcess, matrix: [[f32; 4]; 4], screen_size: Pos2) {
        self.position = Pos3::from_array(read_f32_vec(vp, self.pointer + LOCAL_ORIGIN, 3).as_slice().try_into().unwrap());
        self.position_2d = world_to_screen(matrix, self.position, screen_size);
    }

    pub fn update_distance(&mut self, vp: VmmProcess, pos: &Pos3) {
        self.distance = distance3d(&self.position, pos);
        self.distance_2d = distance2d(&self.position.to_pos2(), &pos.to_pos2())
    }
    pub fn update_bone_index(&mut self, vp: VmmProcess) {
        let model_pointer = read_u64(vp, self.pointer + STUDIOHDR);
        let studio_hdr = read_u64(vp, model_pointer + 0x8);

        let hitbox_cache = read_u16(vp, studio_hdr + 0x34) as u64;
        let hitbox_array = studio_hdr + ((hitbox_cache & 0xFFFE) << (4 * (hitbox_cache & 1)));

        let index_cache = read_u16(vp, hitbox_array + 0x4);
        let hitbox_index = (index_cache & 0xFFFE) << (4 * (index_cache & 1));
        // 19 is bone amount we need
        let data = read_mem(vp, hitbox_index as u64 + hitbox_array, 20 * 0x20);
        // println!("{:?}", data.hex_dump());
        let bone_index: Vec<u16> = data.chunks_exact(0x20)
            .map(|chunk| u16::from_le_bytes(chunk[..2].try_into().unwrap()))
            .collect();

        // println!("{} -> {:?}", self.pointer, bone_index);
        /*        if bone_index.iter().any(|&x| x > 240) {
                    Err(DataError::BoneError)
                };*/

        if self.status.character == Character::Bloodhound {
            self.hitbox.head.index = bone_index[0] as usize;
            self.hitbox.neck.index = bone_index[1] as usize;
            self.hitbox.upper_chest.index = bone_index[2] as usize;
            self.hitbox.lower_chest.index = bone_index[3] as usize;
            self.hitbox.stomach.index = bone_index[4] as usize;
            self.hitbox.hip.index = bone_index[5] as usize;
            self.hitbox.left_shoulder.index = bone_index[6] as usize;
            self.hitbox.left_elbow.index = bone_index[7] as usize;
            self.hitbox.left_hand.index = bone_index[19] as usize;
            self.hitbox.right_shoulder.index = bone_index[8] as usize;
            self.hitbox.right_elbow.index = bone_index[9] as usize;
            self.hitbox.right_hand.index = bone_index[10] as usize;
            self.hitbox.left_thigh.index = bone_index[11] as usize;
            self.hitbox.left_knee.index = bone_index[12] as usize;
            self.hitbox.left_foot.index = bone_index[13] as usize;
            self.hitbox.right_thigh.index = bone_index[15] as usize;
            self.hitbox.right_knee.index = bone_index[16] as usize;
            self.hitbox.right_foot.index = bone_index[17] as usize;
        } else {
            self.hitbox.head.index = bone_index[0] as usize;
            self.hitbox.neck.index = bone_index[1] as usize;
            self.hitbox.upper_chest.index = bone_index[2] as usize;
            self.hitbox.lower_chest.index = bone_index[3] as usize;
            self.hitbox.stomach.index = bone_index[4] as usize;
            self.hitbox.hip.index = bone_index[5] as usize;
            self.hitbox.left_shoulder.index = bone_index[6] as usize;
            self.hitbox.left_elbow.index = bone_index[7] as usize;
            self.hitbox.left_hand.index = bone_index[8] as usize;
            self.hitbox.right_shoulder.index = bone_index[9] as usize;
            self.hitbox.right_elbow.index = bone_index[10] as usize;
            self.hitbox.right_hand.index = bone_index[11] as usize;
            self.hitbox.left_thigh.index = bone_index[12] as usize;
            self.hitbox.left_knee.index = bone_index[13] as usize;
            self.hitbox.left_foot.index = bone_index[14] as usize;
            self.hitbox.right_thigh.index = bone_index[16] as usize;
            self.hitbox.right_knee.index = bone_index[17] as usize;
            self.hitbox.right_foot.index = bone_index[18] as usize;
        };

        // Ok(())
    }

    pub fn update_bone_position(&mut self, vp: VmmProcess, matrix: [[f32; 4]; 4], screen_size: Pos2) {
        let vec_abs_origin: [f32; 3] = read_f32_vec(vp, self.pointer + ABS_VECTORORIGIN, 3).as_slice().try_into().unwrap();

        self.position = Pos3::from_array(vec_abs_origin);
        self.position_2d = world_to_screen(matrix, self.position, screen_size);
        // float: 4 * matrix: 12 * bone: 200
        let data = read_mem(vp, self.bone_pointer, 4 * 12 * 240);
        // println!("{:?}", data.hex_dump());

        let mut f32_num: Vec<f32> = Vec::with_capacity(12 * 240);

        for chunk in data.chunks_exact(4) {
            let mut array: [u8; 4] = [0; 4];
            array.copy_from_slice(chunk);
            let array = f32::from_le_bytes(array);
            f32_num.push(array);
        }

        let matrix: Vec<[[f32; 4]; 3]> = f32_num
            .chunks_exact(4)// [f32; 4]
            .map(|chunk| {
                let mut array: [f32; 4] = [0.0; 4]; // init
                array.copy_from_slice(chunk);
                array
            })
            .collect::<Vec<[f32; 4]>>()
            .chunks_exact(3) // [[f32; 4]; 3]
            .map(|chunk| {
                let mut matrix: [[f32; 4]; 3] = [[0.0; 4]; 3]; // init
                for (i, item) in chunk.iter().enumerate() {
                    matrix[i] = *item;
                }
                matrix
            })
            .collect();
        if self.hitbox.head.index > 240 { return; }
        // println!("name -> {}", self.status.name);
        self.hitbox.head.position = Pos3 {
            x: matrix[self.hitbox.head.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.head.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.head.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.neck.position = Pos3 {
            x: matrix[self.hitbox.neck.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.neck.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.neck.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.upper_chest.position = Pos3 {
            x: matrix[self.hitbox.upper_chest.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.upper_chest.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.upper_chest.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.lower_chest.position = Pos3 {
            x: matrix[self.hitbox.lower_chest.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.lower_chest.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.lower_chest.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.stomach.position = Pos3 {
            x: matrix[self.hitbox.stomach.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.stomach.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.stomach.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.hip.position = Pos3 {
            x: matrix[self.hitbox.hip.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.hip.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.hip.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.left_shoulder.position = Pos3 {
            x: matrix[self.hitbox.left_shoulder.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.left_shoulder.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.left_shoulder.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.left_elbow.position = Pos3 {
            x: matrix[self.hitbox.left_elbow.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.left_elbow.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.left_elbow.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.left_hand.position = Pos3 {
            x: matrix[self.hitbox.left_hand.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.left_hand.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.left_hand.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.right_shoulder.position = Pos3 {
            x: matrix[self.hitbox.right_shoulder.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.right_shoulder.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.right_shoulder.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.right_elbow.position = Pos3 {
            x: matrix[self.hitbox.right_elbow.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.right_elbow.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.right_elbow.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.right_hand.position = Pos3 {
            x: matrix[self.hitbox.right_hand.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.right_hand.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.right_hand.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.left_thigh.position = Pos3 {
            x: matrix[self.hitbox.left_thigh.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.left_thigh.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.left_thigh.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.left_knee.position = Pos3 {
            x: matrix[self.hitbox.left_knee.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.left_knee.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.left_knee.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.left_foot.position = Pos3 {
            x: matrix[self.hitbox.left_foot.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.left_foot.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.left_foot.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.right_thigh.position = Pos3 {
            x: matrix[self.hitbox.right_thigh.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.right_thigh.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.right_thigh.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.right_knee.position = Pos3 {
            x: matrix[self.hitbox.right_knee.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.right_knee.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.right_knee.index][2][3] + vec_abs_origin[2],
        };
        self.hitbox.right_foot.position = Pos3 {
            x: matrix[self.hitbox.right_foot.index][0][3] + vec_abs_origin[0],
            y: matrix[self.hitbox.right_foot.index][1][3] + vec_abs_origin[1],
            z: matrix[self.hitbox.right_foot.index][2][3] + vec_abs_origin[2],
        };
    }

    pub fn update_bone_position_2d(&mut self, matrix: [[f32; 4]; 4], screen_size: Pos2) {
        let mut bones = [
            &mut self.hitbox.head,
            &mut self.hitbox.neck,
            &mut self.hitbox.upper_chest,
            &mut self.hitbox.lower_chest,
            &mut self.hitbox.stomach,
            &mut self.hitbox.hip,
            &mut self.hitbox.left_shoulder,
            &mut self.hitbox.left_elbow,
            &mut self.hitbox.left_hand,
            &mut self.hitbox.right_shoulder,
            &mut self.hitbox.right_elbow,
            &mut self.hitbox.right_hand,
            &mut self.hitbox.left_thigh,
            &mut self.hitbox.left_knee,
            &mut self.hitbox.left_foot,
            &mut self.hitbox.right_thigh,
            &mut self.hitbox.right_knee,
            &mut self.hitbox.right_foot,
        ];

        for bone in bones.iter_mut() {
            bone.position_2d = world_to_screen(matrix, bone.position, screen_size);
        };
    }

    pub fn get_nearest_bone(&self, screen_center: Pos2) -> Bone {
        let mut last_distance: f32 = f32::MAX;
        let mut nearest_bone: Bone = Bone::default();
        let mut bones = [
            &self.hitbox.head,
            &self.hitbox.neck,
            &self.hitbox.upper_chest,
            &self.hitbox.lower_chest,
            &self.hitbox.stomach,
            &self.hitbox.hip,
        ];

        for bone in bones.iter() {
            if bone.position_2d.distance(screen_center).abs() < last_distance {
                nearest_bone = **bone;
                last_distance = bone.position_2d.distance(screen_center).abs();
            }
        }
        nearest_bone
    }

    pub fn hitbox_check(&self, screen_center: Pos2, hitbox_size: f32) -> bool {
        self.get_nearest_bone(screen_center).position_2d.distance(screen_center) < hitbox_size
    }
}

pub fn get_button_state(mut button: i32, vp: VmmProcess, base: u64) -> i32 {
    button = button + 1;
    let a0 = read_i32(vp, base + INPUT_SYSTEM + ((button >> 5) * 4) as u64 + 0xb0);

    // println!("data -> {:?}", read_mem(vp, base + INPUT_SYSTEM + 0xb0 + ((button >> 5) * 4) as u64, 0x30).hex_dump());
    // println!("addr {button} -> {:x} state -> {a0}", base + INPUT_SYSTEM + ((button >> 5) * 4) as u64 + 0xb0);
    return (a0 >> (button & 31)) & 1;
}


#[named_constants]
#[repr(u8)]
pub enum Item {
    None,

    // Weapon
    R301,
    Sentinel,
    Bocek,
    Alternator,
    RE45,
    ChargeRifle,
    Devotion,
    Longbow,
    Havoc,
    EVA8Auto,
    Flatline,
    Hemlok,
    Kraber,
    G7Scout,
    LStar,
    Mastiff,
    Mozambique,
    Prowler,
    PK,
    R99,
    P2020,
    Spitfire,
    TripleTake,
    Wingman,
    Volt,
    Repeater,
    Rampage,
    CAR,

    // Ammo
    LightRounds,
    EnergyAmmo,
    ShotgunShells,
    HeavyRounds,
    SniperAmmo,
    Arrows,

    // Meds
    UltAccel,
    PhoenixKit,
    MedKit,
    Syringe,
    Battery,
    ShieldCell,

    // Equipment
    HelmetLv1,
    HelmetLv2,
    HelmetLv3,
    HelmetLv4,
    BodyArmorLv1,
    BodyArmorLv2,
    BodyArmorLv3,
    BodyArmorLv4,
    EvoShieldLv0,
    EvoShieldLv1,
    EvoShieldLv2,
    EvoShieldLv3,
    EvoShieldLv4,
    KnockdownShieldLv1,
    KnockdownShieldLv2,
    KnockdownShieldLv3,
    KnockdownShieldLv4,
    BackpackLv1,
    BackpackLv2,
    BackpackLv3,
    BackpackLv4,

    // Grenades
    Thermite,
    FragGrenade,
    ArcStar,

    // Sights
    HcogClassic,
    HcogBruiser,
    HcogRanger,
    Holo,
    VariableHolo,
    VariableAOG,
    DigiThreat,
    Sniper,
    VariableSniper,
    DigiSniperThreat,

    // Attachments
    BarrelStabilizerLv1,
    BarrelStabilizerLv2,
    BarrelStabilizerLv3,
    BarrelStabilizerLv4,
    LaserSightLv1,
    LaserSightLv2,
    LaserSightLv3,
    LaserSightLv4,
    LightMagazineLv1,
    LightMagazineLv2,
    LightMagazineLv3,
    LightMagazineLv4,
    HeavyMagazineLv1,
    HeavyMagazineLv2,
    HeavyMagazineLv3,
    HeavyMagazineLv4,
    EnergyMagazineLv1,
    EnergyMagazineLv2,
    EnergyMagazineLv3,
    EnergyMagazineLv4,
    SniperMagazineLv1,
    SniperMagazineLv2,
    SniperMagazineLv3,
    SniperMagazineLv4,
    ShotgunBoltLv1,
    ShotgunBoltLv2,
    ShotgunBoltLv3,
    ShotgunBoltLv4,
    StandardStockLv1,
    StandardStockLv2,
    StandardStockLv3,
    SniperStockLv1,
    SniperStockLv2,
    SniperStockLv3,

    // Hop-ups
    EpicHopUp0,
    EpicHopUp3,
    LegendaryHopUp0,
    LegendaryHopUp4,

    // Misc
    Keycard,
    TreasurePack,
    HeatShield,
    MobileRespawn,
    MrvnArm,
    GoldenTicket,
    BannerCrafting,
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub enum Character {
    Table,
    Ash,
    Ballistic,
    Bangalore,
    Bloodhound,
    Catalyst,
    Caustic,
    Crypto,
    Fuse,
    Gibraltar,
    Horizon,
    Lifeline,
    Loba,
    MadMaggie,
    Mirage,
    Newcastle,
    Octane,
    Pathfinder,
    Rampart,
    Revenant,
    Seer,
    Valkyrie,
    Vantage,
    Wattson,
    Wraith,
    #[default]
    None,
}

#[derive(Default, Clone, Debug)]
pub struct CharacterType {
    pub table: HashMap<u16, Character>,
}

impl CharacterType {
    pub fn initialize_character_type(&mut self) {
        let character_data: [(Character, Vec<u16>); 23] = [
            (Character::Ash, vec![1, 2]),
            (Character::Ballistic, vec![3, 4]),
            (Character::Bangalore, vec![5, 6]),
            (Character::Bloodhound, vec![7, 8]),
            (Character::Catalyst, vec![9, 10]),
            (Character::Caustic, vec![11, 12]),
            (Character::Crypto, vec![13, 14]),
            (Character::Horizon, vec![15, 16]),
            (Character::Lifeline, vec![17, 18]),
            (Character::Loba, vec![19, 20]),
            (Character::MadMaggie, vec![21, 22]),
            (Character::Mirage, vec![23, 24]),
            (Character::Newcastle, vec![25, 26]),
            (Character::Octane, vec![27, 28]),
            (Character::Pathfinder, vec![29, 30]),
            (Character::Rampart, vec![31, 32]),
            (Character::Revenant, vec![4173]),
            (Character::Seer, vec![35, 36]),
            (Character::Valkyrie, vec![37, 38]),
            (Character::Vantage, vec![39, 40]),
            (Character::Wattson, vec![41, 42]),
            (Character::Wraith, vec![43, 44]),
            (Character::None, Vec::new()),
        ];

        for (character, values) in &character_data {
            for value in values {
                self.table.insert(*value, *character);
            }
        }
    }

    pub fn check_character_type(&self, value: u16) -> Character {
        *self.table.get(&value).unwrap_or(&Character::None)
    }
}


pub struct DataTable {
    pub character: CharacterType,

}


#[named_constants]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum InputSystem {
    KEY_NONE,
    KEY_0 = 0,
    KEY_1 = 1,
    KEY_2 = 2,
    KEY_3 = 3,
    KEY_4 = 4,
    KEY_5 = 5,
    KEY_6 = 6,
    KEY_7 = 7,
    KEY_8 = 8,
    KEY_9 = 9,
    KEY_A = 10,
    KEY_B = 11,
    KEY_C = 12,
    KEY_D = 13,
    KEY_E = 14,
    KEY_F = 15,
    KEY_G = 16,
    KEY_H = 17,
    KEY_I = 18,
    KEY_J = 19,
    KEY_K = 20,
    KEY_L = 21,
    KEY_M = 22,
    KEY_N = 23,
    KEY_O = 24,
    KEY_P = 25,
    KEY_Q = 26,
    KEY_R = 27,
    KEY_S = 28,
    KEY_T = 29,
    KEY_U = 30,
    KEY_V = 31,
    KEY_W = 32,
    KEY_X = 33,
    KEY_Y = 34,
    KEY_Z = 35,
    KEY_PAD_0 = 36,
    KEY_PAD_1 = 37,
    KEY_PAD_2 = 38,
    KEY_PAD_3 = 39,
    KEY_PAD_4 = 40,
    KEY_PAD_5 = 41,
    KEY_PAD_6 = 42,
    KEY_PAD_7 = 43,
    KEY_PAD_8 = 44,
    KEY_PAD_9 = 45,
    KEY_PAD_DIVIDE = 46,
    KEY_PAD_MULTIPLY = 47,
    KEY_PAD_MINUS = 48,
    KEY_PAD_PLUS = 49,
    KEY_PAD_ENTER = 50,
    KEY_PAD_DECIMAL = 51,
    KEY_LBRACKET = 52,
    KEY_RBRACKET = 53,
    KEY_SEMICOLON = 54,
    KEY_APOSTROPHE = 55,
    KEY_BACKQUOTE = 56,
    KEY_COMMA = 57,
    KEY_PERIOD = 58,
    KEY_SLASH = 59,
    KEY_BACKSLASH = 60,
    KEY_MINUS = 61,
    KEY_EQUAL = 62,
    KEY_ENTER = 63,
    KEY_SPACE = 64,
    KEY_BACKSPACE = 65,
    KEY_TAB = 66,
    KEY_CAPSLOCK = 67,
    KEY_NUMLOCK = 68,
    KEY_ESCAPE = 69,
    KEY_SCROLLLOCK = 70,
    KEY_INSERT = 71,
    KEY_DELETE = 72,
    KEY_HOME = 73,
    KEY_END = 74,
    KEY_PAGEUP = 75,
    KEY_PAGEDOWN = 76,
    KEY_BREAK = 77,
    KEY_LSHIFT = 78,
    KEY_RSHIFT = 79,
    KEY_LALT = 80,
    KEY_RALT = 81,
    KEY_LCONTROL = 82,
    KEY_RCONTROL = 83,
    KEY_LWIN = 84,
    KEY_RWIN = 85,
    KEY_APP = 86,
    KEY_UP = 87,
    KEY_LEFT = 88,
    KEY_DOWN = 89,
    KEY_RIGHT = 90,
    KEY_F1 = 91,
    KEY_F2 = 92,
    KEY_F3 = 93,
    KEY_F4 = 94,
    KEY_F5 = 95,
    KEY_F6 = 96,
    KEY_F7 = 97,
    KEY_F8 = 98,
    KEY_F9 = 99,
    KEY_F10 = 100,
    KEY_F11 = 101,
    KEY_F12 = 102,
    KEY_CAPSLOCKTOGGLE = 103,
    KEY_NUMLOCKTOGGLE = 104,

    MOUSE_LEFT = 107,
    MOUSE_RIGHT = 108,
    MOUSE_MIDDLE = 109,
    MOUSE_4 = 110, // side down

    MOUSE_5 = 111, // side up

    // XBox 360
    KEY_XBUTTON_UP = 242, // POV buttons

    KEY_XBUTTON_RIGHT = 243,
    KEY_XBUTTON_DOWN = 244,
    KEY_XBUTTON_LEFT = 245,

    KEY_XBUTTON_A = 114,
    KEY_XBUTTON_B = 115,
    KEY_XBUTTON_X = 116,
    KEY_XBUTTON_Y = 117,

    KEY_XBUTTON_LEFT_SHOULDER = 118,
    KEY_XBUTTON_RIGHT_SHOULDER = 119,

    KEY_XBUTTON_BACK = 120,
    KEY_XBUTTON_START = 121,

    KEY_XBUTTON_STICK1 = 122, // left stick down
    KEY_XBUTTON_STICK2 = 123, // right stick down

    KEY_XBUTTON_LTRIGGER = 125, // ZAXIS POSITIVE
    KEY_XBUTTON_RTRIGGER = 126, // ZAXIS NEGATIVE
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyData {
    pub data: ContinuingData,
}

impl Default for KeyData {
    fn default() -> Self {
        KeyData { data: ContinuingData::new(Vec::new()) }
    }
}

impl KeyData {
    pub fn update_key_state(&mut self, vp: VmmProcess, base: u64) {
        self.data = ContinuingData::new(read_mem(vp, base + INPUT_SYSTEM + 0xb0, 0x20));

        // for i in 0..255 {
        //     self.data[i] = (data.read_i32(((i >> 5) * 4) as u64) >> (i & 31)) & 1
        // }
    }
    pub fn get_key_state(&self, key: u8) -> bool {
        let value = (InputSystem(key).0 + 1);
        (self.data.read_i32(((value >> 5) * 4) as u64) >> (value & 31)) & 1 == 1
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Map {
    CanyonlandsMu3,
    DesertlandsMu3,
    OlympusMu2,
    TropicIslandMu1,
    Lobby,
}
impl Map {
    pub fn get_map(v: String) -> Map {
        match v.as_str() {
            "mp_lobby" => { Map::Lobby },
            _ => { Map::Lobby }
        }
    }

    pub fn get_position(&self) -> Pos2 {
        match self {
            Map::CanyonlandsMu3 => { Pos2::new(1.0, 1.0) }
            _ => { Pos2::new(1.0, 1.0) }
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Pitch {
    pub view: f32,
    pub launch: f32,
}

pub fn launch2view(pitches: &[Pitch], launch: f32) -> f32 {
    if pitches.len() < 2 {
        return launch;
    }

    let mut low = 0;
    let mut high = pitches.len() - 1;
    while low + 1 != high {
        let middle = (low + high) / 2;
        let entry = match pitches.get(middle) { Some(e) => e, None => return launch };
        if launch < entry.launch {
            high = middle;
        }
        else {
            low = middle;
        }
    }

    let low = match pitches.get(low) { Some(e) => e, None => return launch };
    let high = match pitches.get(high) { Some(e) => e, None => return launch };

    let fraction = (launch - low.launch) / (high.launch - low.launch);
    low.view + fraction * (high.view - low.view)
}

pub fn view2launch(pitches: &[Pitch], view: f32) -> f32 {
    if pitches.len() < 2 {
        return view;
    }

    let mut low = 0;
    let mut high = pitches.len() - 1;
    while low + 1 != high {
        let middle = (low + high) / 2;
        let entry = match pitches.get(middle) { Some(e) => e, None => return view };
        if view < entry.view {
            high = middle;
        }
        else {
            low = middle;
        }
    }

    let low = match pitches.get(low) { Some(e) => e, None => return view };
    let high = match pitches.get(high) { Some(e) => e, None => return view };

    let fraction = (view - low.view) / (high.view - low.view);
    low.launch + fraction * (high.launch - low.launch)
}

// Thermite and Frag Grenades
pub static GRENADE_PITCHES: [Pitch; 49] = [
    Pitch { view: -1.5533, launch: -1.3990 }, // 89
    Pitch { view: -1.4837, launch: -1.3267 }, // 85
    Pitch { view: -1.3962, launch: -1.2433 }, // 80
    Pitch { view: -1.3092, launch: -1.1534 }, // 75
    Pitch { view: -1.2217, launch: -1.0779 }, // 70
    Pitch { view: -1.1347, launch: -0.9783 }, // 65
    Pitch { view: -1.0472, launch: -0.8977 }, // 60
    Pitch { view: -0.9602, launch: -0.8104 }, // 55
    Pitch { view: -0.8727, launch: -0.7268 }, // 50
    Pitch { view: -0.7857, launch: -0.6375 }, // 45
    Pitch { view: -0.6981, launch: -0.5439 }, // 40
    Pitch { view: -0.6112, launch: -0.4688 }, // 35
    Pitch { view: -0.5236, launch: -0.3880 }, // 30
    Pitch { view: -0.3491, launch: -0.2050 }, // 25
    Pitch { view: -0.3491, launch: -0.2050 }, // 20
    Pitch { view: -0.2615, launch: -0.1165 }, // 15
    Pitch { view: -0.1746, launch: -0.0421 }, // 10
    Pitch { view: -0.0870, launch: 0.0644 },  //  5
    Pitch { view: -0.0001, launch: 0.1403 },  //  0
    Pitch { view: 0.0875, launch: 0.2358 },   // -5
    Pitch { view: 0.1745, launch: 0.3061 },   //-10
    Pitch { view: 0.2620, launch: 0.3753 },   //-15
    Pitch { view: 0.3490, launch: 0.4684 },   //-20
    Pitch { view: 0.4365, launch: 0.5343 },   //-25
    Pitch { view: 0.5235, launch: 0.6238 },   //-30
    Pitch { view: 0.6110, launch: 0.6865 },   //-35
    Pitch { view: 0.6979, launch: 0.7756 },   //-40
    Pitch { view: 0.7331, launch: 0.7968 },   //-42
    Pitch { view: 0.7682, launch: 0.8341 },   //-44
    Pitch { view: 0.8027, launch: 0.8771 },   //-46
    Pitch { view: 0.8379, launch: 0.9038 },   //-48
    Pitch { view: 0.8727, launch: 0.9382 },   //-50
    Pitch { view: 0.9079, launch: 0.9620 },   //-52
    Pitch { view: 0.9424, launch: 1.0048 },   //-54
    Pitch { view: 0.9775, launch: 1.0333 },   //-56
    Pitch { view: 1.0121, launch: 1.0561 },   //-58
    Pitch { view: 1.0472, launch: 1.0987 },   //-60
    Pitch { view: 1.0824, launch: 1.1217 },   //-62
    Pitch { view: 1.1175, launch: 1.1628 },   //-64
    Pitch { view: 1.1520, launch: 1.1868 },   //-66
    Pitch { view: 1.1866, launch: 1.2239 },   //-68
    Pitch { view: 1.2217, launch: 1.2555 },   //-70
    Pitch { view: 1.2563, launch: 1.2859 },   //-72
    Pitch { view: 1.2913, launch: 1.3156 },   //-74
    Pitch { view: 1.3264, launch: 1.3470 },   //-76
    Pitch { view: 1.3615, launch: 1.3822 },   //-78
    Pitch { view: 1.3973, launch: 1.4108 },   //-80
    Pitch { view: 1.4837, launch: 1.4919 },   //-85
    Pitch { view: 1.5533, launch: 1.5546 },   //-89
];

// Arc Star
pub static ARC_PITCHES: [Pitch; 19] = [
    Pitch { view: -1.5533, launch: -1.5198 },
    Pitch { view: -1.3967, launch: -1.3672 },
    Pitch { view: -1.2222, launch: -1.1974 },
    Pitch { view: -1.0477, launch: -1.0260 },
    Pitch { view: -0.8731, launch: -0.8550 },
    Pitch { view: -0.6986, launch: -0.6848 },
    Pitch { view: -0.5241, launch: -0.5129 },
    Pitch { view: -0.3496, launch: -0.3416 },
    Pitch { view: -0.1572, launch: -0.1484 },
    Pitch { view: 0.0000, launch: 0.0080 },
    Pitch { view: 0.1751, launch: 0.1800 },
    Pitch { view: 0.3496, launch: 0.3520 },
    Pitch { view: 0.5241, launch: 0.5234 },
    Pitch { view: 0.6992, launch: 0.6978 },
    Pitch { view: 0.8727, launch: 0.8710 },
    Pitch { view: 1.0472, launch: 1.0453 },
    Pitch { view: 1.2218, launch: 1.2201 },
    Pitch { view: 1.3963, launch: 1.3956 },
    Pitch { view: 1.5533, launch: 1.5533 },
];

// Grenadier Thermite and Frag Grenades
pub static GRENADIER_GRENADE_PITCHES: [Pitch; 19] = [
    Pitch { view: -1.5533, launch: -1.3991 },
    Pitch { view: -1.3973, launch: -1.2456 },
    Pitch { view: -1.2227, launch: -1.0736 },
    Pitch { view: -1.0477, launch: -0.9010 },
    Pitch { view: -0.8737, launch: -0.7293 },
    Pitch { view: -0.6992, launch: -0.5562 },
    Pitch { view: -0.5247, launch: -0.3832 },
    Pitch { view: -0.3507, launch: -0.2101 },
    Pitch { view: -0.1762, launch: -0.0358 },
    Pitch { view: 0.0000, launch: 0.1406 },
    Pitch { view: 0.1745, launch: 0.2984 },
    Pitch { view: 0.3496, launch: 0.4565 },
    Pitch { view: 0.5247, launch: 0.6157 },
    Pitch { view: 0.6987, launch: 0.7741 },
    Pitch { view: 0.8732, launch: 0.9331 },
    Pitch { view: 1.0477, launch: 1.0924 },
    Pitch { view: 1.2222, launch: 1.2519 },
    Pitch { view: 1.3973, launch: 1.4120 },
    Pitch { view: 1.5533, launch: 1.5548 },
];

// Grenadier Arc Star
pub static GRENADIER_ARC_PITCHES: [Pitch; 19] = [
    Pitch { view: -1.5533, launch: -1.5193 },
    Pitch { view: -1.3973, launch: -1.3657 },
    Pitch { view: -1.2222, launch: -1.1931 },
    Pitch { view: -1.0477, launch: -1.0210 },
    Pitch { view: -0.8731, launch: -0.8485 },
    Pitch { view: -0.6986, launch: -0.6759 },
    Pitch { view: -0.5241, launch: -0.5034 },
    Pitch { view: -0.3496, launch: -0.3297 },
    Pitch { view: -0.1751, launch: -0.1554 },
    Pitch { view: 0.0000, launch: 0.0190 },
    Pitch { view: 0.1763, launch: 0.1916 },
    Pitch { view: 0.3502, launch: 0.3624 },
    Pitch { view: 0.5241, launch: 0.5339 },
    Pitch { view: 0.6992, launch: 0.7065 },
    Pitch { view: 0.8737, launch: 0.8792 },
    Pitch { view: 1.0480, launch: 1.0518 },
    Pitch { view: 1.2220, launch: 1.2251 },
    Pitch { view: 1.3965, launch: 1.3976 },
    Pitch { view: 1.5533, launch: 1.5534 },
];
