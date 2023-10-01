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
    pub width: f32,
    pub left: Pos2,
    pub right: Pos2,
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
    pub last_crosshair_target_time: f32,
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
        self.health = read_u16(vp, addr + HEALTH);
        self.max_health = read_u16(vp, addr + MAX_HEALTH);

        self.shield = read_u16(vp, addr + SHIELD);
        self.max_shield = read_u16(vp, addr + MAX_SHIELD);

        self.armor_type = read_u16(vp, addr + ARMOR_TYPE);
        self.helmet_type = read_u16(vp, addr + HELMET_TYPE);


        self.team = read_u16(vp, addr + TEAM_NUM);
        self.team_index = read_u16(vp, addr + TEAM_MEMBER_INDEX);

        self.dead = read_u16(vp, addr + LIFE_STATE);
        self.knocked = read_u16(vp, addr + BLEED_OUT_STATE);
        let name_ptr = read_u64(vp, base + NAME_LIST + (index - 1) * 0x10);
        self.name = read_string(vp, name_ptr);
        // println!("squad id -> {}", self.team_index)

    }

    pub fn update(&mut self, vp: VmmProcess, addr: &u64) {
        self.health = read_u16(vp, addr + HEALTH); // 0x036c
        self.max_health = read_u16(vp, addr + MAX_HEALTH); // 0x04a8

        self.shield = read_u16(vp, addr + SHIELD); // 0x01a0
        self.max_shield = read_u16(vp, addr + MAX_SHIELD); // 0x01a4

        self.armor_type = read_u16(vp, addr + ARMOR_TYPE); // 0x45c4
        self.helmet_type = read_u16(vp, addr + HELMET_TYPE); // 0x45c0
        self.skin = read_u16(vp, addr + CURRENT_FRAMEMODEL_INDEX); // 0x00d8
        // let player_data_ptr = read_u64(vp, addr + PLAYER_DATA);
        // let player_datas = read_u16(vp, player_data_ptr + LEGENDARY_MODEL_INDEX);
        // let player_data = read_mem(vp, addr + PLAYER_DATA, 0x100);
        self.last_visible_time = read_f32(vp, addr + LAST_VISIBLE_TIME); // 0x19B0
        self.last_crosshair_target_time = read_f32(vp, addr + LAST_VISIBLE_TIME + 0x8); // 0x19B0
        self.dead = read_u16(vp, addr + LIFE_STATE); // 0x06c8
        self.knocked = read_u16(vp, addr + BLEED_OUT_STATE); // 0x26a0
        let mut da = CharacterType::default();
        da.initialize_character_type();
        self.character = da.check_character_type(self.skin);
        self.platform_id = read_u64(vp, addr + PLATFORM_USER_ID); // 0x2508
        // let da = read_mem(vp, addr + LAST_VISIBLE_TIME, 0x30);
        // info!("ptr -> {:x} data -> {} direct -> {:?}", player_data_ptr, player_datas, player_data.hex_dump())
        // info!("last visible time -> {}", self.last_visible_time);
        // info!("data -> {:?}", da.hex_dump());
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
}

impl LocalPlayer {
    pub fn update_pointer(&mut self, vp: VmmProcess, base: u64) {
        self.pointer = read_u64(vp, base + LOCAL_PLAYER);
        self.render_pointer = read_u64(vp, base + VIEW_RENDER);
        self.matrix_pointer = read_u64(vp, self.render_pointer + VIEW_MATRIX);
    }

    pub fn update_position(&mut self, vp: VmmProcess) {
        self.position = Pos3::from_array(read_f32_vec(vp, self.pointer + LOCAL_ORIGIN, 3).as_slice().try_into().unwrap());
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
        self.pitch = read_f32(vp, self.pointer + VIEW_ANGLE);
        self.yaw = read_f32(vp, self.pointer + VIEW_ANGLE + 0x4);
    }
    pub fn set_pitch(&mut self, vp: VmmProcess, pitch: f32) {
        write_f32(vp, self.pointer + VIEW_ANGLE, pitch);
    }
    pub fn set_yaw(&mut self, vp: VmmProcess, yaw: f32) {
        write_f32(vp, self.pointer + VIEW_ANGLE + 0x4, yaw);
    }
}

pub enum DataError {
    BoneError
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

    pub fn box_esp(&self, ptr: Painter) {

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
                        Stroke::new(2.0, Color32::WHITE)));
        ptr.add(
            Shape::line(hand,
                        Stroke::new(2.0, Color32::WHITE)));
        ptr.add(
            Shape::line(leg,
                        Stroke::new(2.0, Color32::WHITE)));
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
        ptr.text(self.hitbox.lower_chest.position_2d,
                 Align2::CENTER_BOTTOM,
                 format!("{:?}", self.status.last_visible_time),
                 FontId::default(),
                 Color32::BLUE);

        ptr.text(self.hitbox.left_elbow.position_2d,
                 Align2::CENTER_BOTTOM,
                 format!("{:?}", self.status.last_crosshair_target_time),
                 FontId::default(),
                 Color32::RED);


    }
    pub fn target_line(&self, ptr: Painter) {
        ptr.line_segment(
            [self.hitbox.head.position_2d, Pos2::new(2560.0 / 2.0, 1440.0 / 2.0)],
            Stroke::new(4.0, Color32::RED));
    }

    pub fn update_position(&mut self, vp: VmmProcess, matrix: [[f32; 4]; 4]) {
        self.position = Pos3::from_array(read_f32_vec(vp, self.pointer + LOCAL_ORIGIN, 3).as_slice().try_into().unwrap());
        self.position_2d = world_to_screen(matrix, self.position, Pos2 {x: 2560.0, y: 1440.0});
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

    pub fn update_bone_position(&mut self, vp: VmmProcess) {
        let vec_abs_origin: [f32; 3] = read_f32_vec(vp, self.pointer + ABS_VECTORORIGIN, 3).as_slice().try_into().unwrap();

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
}

pub fn get_button_state(mut button: i32, vp: VmmProcess, base: u64) -> i32 {
    button = button + 1;
    let a0 = read_i32(vp, base + INPUT_SYSTEM + ((button >> 5) * 4) as u64 + 0xb0);
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