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

#[derive(Debug, Copy, Clone, Default)]
pub struct Pos3 {
    pub x: f32,

    pub y: f32,

    pub z: f32,
}

impl Pos3 {
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
        let data = ContinuingData::new(read_mem(vp, addr, 0x4600));
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
        let data = ContinuingData::new(read_mem(vp, *addr, 0x4600));
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
        // let local: Vec<f32> = read_f32_vec(vp, self.pointer + LOCAL_ORIGIN, 3).as_slice().try_into().unwrap();
        // let vec: Vec<f32> = read_f32_vec(vp, self.pointer + ABS_VECTORORIGIN, 3).as_slice().try_into().unwrap();
        // println!("local -> {:?}", local);
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
    None
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
            [Pos2 {x: self.position_2d.x + 20.0 , y: self.position_2d.y + 10.0},
                    Pos2 {x: self.position_2d.x + 20.0 , y: self.position_2d.y + 10.0 - self.status.health as f32 / 3.0}],
            Stroke::new( 4.0, Color32::GREEN)
        );

        ptr.line_segment(
            [Pos2 {x: self.position_2d.x + 25.0 , y: self.position_2d.y + 10.0},
                Pos2 {x: self.position_2d.x + 25.0 , y: self.position_2d.y + 10.0 - self.status.shield as f32 / 3.0}],
            Stroke::new( 4.0, Color32::BLUE)
        );
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
        self.distance = distance3d(&self.position, pos)
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
        if self.hitbox.head.index > 240 {return;}
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

    pub fn update_bone_position_2d(&mut self, matrix: [[f32; 4]; 4]) {
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
            bone.position_2d = world_to_screen(matrix, bone.position, Pos2 {x: 2560.0, y: 1440.0});
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
            &self.hitbox.left_shoulder,

            &self.hitbox.right_shoulder,

            &self.hitbox.left_thigh,

            &self.hitbox.right_thigh,

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
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5,
    KEY_6,
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_I,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_M,
    KEY_N,
    KEY_O,
    KEY_P,
    KEY_Q,
    KEY_R,
    KEY_S,
    KEY_T,
    KEY_U,
    KEY_V,
    KEY_W,
    KEY_X,
    KEY_Y,
    KEY_Z = 35,
    KEY_PAD_0 = 36,
    KEY_PAD_1,
    KEY_PAD_2,
    KEY_PAD_3,
    KEY_PAD_4,
    KEY_PAD_5,
    KEY_PAD_6,
    KEY_PAD_7,
    KEY_PAD_8,
    KEY_PAD_9 = 45,
    KEY_PAD_DIVIDE,
    KEY_PAD_MULTIPLY = 47,
    KEY_PAD_MINUS = 48,
    KEY_PAD_PLUS = 49,
    KEY_PAD_ENTER = 50,
    KEY_PAD_DECIMAL = 51,
    KEY_LBRACKET,
    KEY_RBRACKET,
    KEY_SEMICOLON,
    KEY_APOSTROPHE,
    KEY_BACKQUOTE,
    KEY_COMMA,
    KEY_PERIOD,
    KEY_SLASH,
    KEY_BACKSLASH,
    KEY_MINUS,
    KEY_EQUAL,
    KEY_ENTER,
    KEY_SPACE,
    KEY_BACKSPACE,
    KEY_TAB = 66,
    KEY_CAPSLOCK,
    KEY_NUMLOCK = 68,
    KEY_ESCAPE,
    KEY_SCROLLLOCK,
    KEY_INSERT,
    KEY_DELETE,
    KEY_HOME,
    KEY_END,
    KEY_PAGEUP,
    KEY_PAGEDOWN,
    KEY_BREAK,
    KEY_LSHIFT,
    KEY_RSHIFT,
    KEY_LALT = 80,
    KEY_RALT,
    KEY_LCONTROL,
    KEY_RCONTROL,
    KEY_LWIN,
    KEY_RWIN,
    KEY_APP,
    KEY_UP,
    KEY_LEFT,
    KEY_DOWN,
    KEY_RIGHT,
    KEY_F1 = 91,
    KEY_F2,
    KEY_F3,
    KEY_F4,
    KEY_F5,
    KEY_F6,
    KEY_F7,
    KEY_F8,
    KEY_F9,
    KEY_F10,
    KEY_F11,
    KEY_F12 = 102,
    KEY_CAPSLOCKTOGGLE = 103,
    KEY_NUMLOCKTOGGLE = 104,

    MOUSE_LEFT = 107,
    MOUSE_RIGHT = 108,
    MOUSE_MIDDLE  = 109,
    MOUSE_4 = 110, // side down
    MOUSE_5 = 111, // side up

    // XBox 360
    KEY_XBUTTON_UP = 242, // POV buttons
    KEY_XBUTTON_RIGHT = 243,
    KEY_XBUTTON_DOWN = 244,
    KEY_XBUTTON_LEFT = 245,

    KEY_XBUTTON_A = 114, // Buttons
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
#[derive(Debug,Copy, Clone)]
pub struct KeyData {
    pub data: [i32; 255]
}
impl Default for KeyData {
    fn default() -> Self {
        KeyData { data: [0; 255] }
    }
}
impl KeyData {

    pub fn update_key_state(&mut self, vp: VmmProcess, base: u64){
        let data = ContinuingData::new(
            read_mem(vp, base + INPUT_SYSTEM + 0xb0, 0x20));
        for i in 0..255 {
            self.data[i] = (data.read_i32(((i >> 5) * 4) as u64) >> (i & 31)) & 1
        }
    }
    pub fn get_key_state(&self, value: InputSystem) -> bool{
        if self.data[(value.0 + 1) as usize] == 1 {
            true
        } else {
            false
        }
    }
}
