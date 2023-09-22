
use memprocfs::*;
use crate::constants::offsets::*;
use crate::egui_overlay::egui::Pos2;
use crate::function::*;
use crate::math::distance3d;
use crate::mem::*;

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub index: u64,
    pub pointer: u64,
    pub bone_pointer: u64,
    pub hitbox: Hitbox,
    pub status: Status,
    pub position: Pos3,
    pub distance: f32,
}


#[derive(Debug,Copy, Clone, Default)]
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

}

#[derive(Debug, Clone, Default)]
pub struct Status {
    pub dead: u16,
    pub knocked: u16,
    pub health: u16,
    pub max_health: u16,
    pub shield: u16,
    pub max_shield: u16,
    pub team: u16,
    pub name: String,

}

impl Status {
    /// addr -> entity pointer
    pub fn initialize(&mut self, vp: VmmProcess, addr: u64, base: u64, index: u64) {
        self.health = read_u16(vp, addr + HEALTH);
        self.max_health = read_u16(vp, addr + MAX_HEALTH);
        self.shield = read_u16(vp, addr + SHIELD);
        self.max_shield = read_u16(vp, addr + MAX_SHIELD);
        self.team = read_u16(vp, addr + TEAM_NUM);
        self.knocked = read_u16(vp, addr + BLEED_OUT_STATE);
        self.dead = read_u16(vp, addr + LIFE_STATE);
        let name_ptr = read_u64(vp, base + NAME_LIST + (index - 1) * 0x10);
        self.name = read_string(vp, name_ptr);
        // println!("name {index} -> {}", self.name)
    }

    pub fn update(&mut self, vp: VmmProcess, addr: &u64) {
        self.health = read_u16(vp, addr + HEALTH);
        self.max_health = read_u16(vp, addr + MAX_HEALTH);
        self.shield = read_u16(vp, addr + SHIELD);
        self.max_shield = read_u16(vp, addr + MAX_SHIELD);
        self.knocked = read_u16(vp, addr + BLEED_OUT_STATE);
        self.dead = read_u16(vp, addr + LIFE_STATE);
        // println!("name {index} -> {}", self.name)
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
}


impl Player {
    pub fn update_pointer(&mut self, vp: VmmProcess) {
        self.bone_pointer = read_u64(vp, self.pointer + BONE);
    }

    /// ptr -> painter
    pub fn get_bones_position(&self) -> Vec<Pos3>{
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

// 使用迭代器将所有骨骼的位置添加到 res 中
        for bone in bones.iter() {

            res.push(**bone);
        };
        res

    }

    pub fn update_position(&mut self, vp: VmmProcess) {
        self.position = Pos3::from_array(read_f32_vec(vp, self.pointer + LOCAL_ORIGIN, 3).as_slice().try_into().unwrap());
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
        let data = read_mem(vp, hitbox_index as u64 + hitbox_array, 19 * 0x20);
        // println!("{:?}", data.hex_dump());
        let bone_index: Vec<u16> = data.chunks_exact(0x20)
            .map(|chunk| u16::from_le_bytes(chunk[..2].try_into().unwrap()))
            .collect();

        println!("{} -> {:?}", self.pointer, bone_index);
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




