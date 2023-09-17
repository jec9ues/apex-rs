use std::mem::size_of;
use std::ptr::addr_of;
use std::thread::sleep;
use memprocfs::*;
use pretty_hex::PrettyHex;
use crate::constants::offsets::*;
use crate::function::{Pos3, read_bone_from_hitbox};
use crate::mem::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Player {
    pub pointer: u64,
    pub bone_pointer: u64,
    pub hitbox: Hitbox,
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
    pub index: u16,
    pub position: [f32; 3],
}

impl Player {
    /// addr -> entity pointer
    pub fn update_bone_index(&mut self, vp: VmmProcess) {
        let model_pointer = read_u64(vp, self.pointer + STUDIOHDR);
        let studio_hdr = read_u64(vp, model_pointer + 0x8);

        let hitbox_cache = read_u16(vp, studio_hdr + 0x34) as u64;
        let hitbox_array = studio_hdr + ((hitbox_cache & 0xFFFE) << (4 * (hitbox_cache & 1)));

        let index_cache = read_u16(vp, hitbox_array + 0x4);
        let hitbox_index = ((index_cache & 0xFFFE) << (4 * (index_cache & 1)));

        let data = read_mem(vp, hitbox_index as u64 + hitbox_array, 19 * 0x20);
        // println!("{:?}", data.hex_dump());
        let bone_index: Vec<u16> = data.chunks_exact(0x20)
            .map(|chunk| u16::from_le_bytes(chunk[..2].try_into().unwrap()))
            .collect();

        self.hitbox.head.index = bone_index[0];
        self.hitbox.neck.index = bone_index[1];
        self.hitbox.upper_chest.index = bone_index[2];
        self.hitbox.lower_chest.index = bone_index[3];
        self.hitbox.stomach.index = bone_index[4];
        self.hitbox.hip.index = bone_index[5];
        self.hitbox.left_shoulder.index = bone_index[6];
        self.hitbox.left_elbow.index = bone_index[7];
        self.hitbox.left_hand.index = bone_index[8];
        self.hitbox.right_shoulder.index = bone_index[9];
        self.hitbox.right_elbow.index = bone_index[10];
        self.hitbox.right_hand.index = bone_index[11];
        self.hitbox.left_thigh.index = bone_index[12];
        self.hitbox.left_knee.index = bone_index[13];
        self.hitbox.left_foot.index = bone_index[14];
        self.hitbox.right_thigh.index = bone_index[16];
        self.hitbox.right_knee.index = bone_index[17];
        self.hitbox.right_foot.index = bone_index[18];
    }

    pub fn update_pointer(&mut self, vp: VmmProcess) {
        self.bone_pointer = read_u64(vp, self.pointer + BONE);
    }

    pub fn update_bone_position(&mut self, vp: VmmProcess) {
        let vec_abs_origin: [f32; 3] = read_f32_vec(vp, self.pointer + ABS_VECTORORIGIN, 3).as_slice().try_into().unwrap();
        const BUFFER_SIZE: u64 = size_of::<f32>() as u64;
        let matrix: [[f32; 4]; 3] = read_f32_vec(vp, self.bone_pointer + self.hitbox.head.index as u64 * (12 * BUFFER_SIZE), 12)
            .chunks_exact(4)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<[f32; 4]>>()
            .try_into()
            .unwrap();

        self.hitbox.head.position = [matrix[0][3] + vec_abs_origin[0], matrix[1][3] + vec_abs_origin[1], matrix[2][3] + vec_abs_origin[2]];
    }

    pub fn update_bone_position_test(&mut self, vp: VmmProcess) {
        let vec_abs_origin: [f32; 3] = read_f32_vec(vp, self.pointer + ABS_VECTORORIGIN, 3).as_slice().try_into().unwrap();
        const BUFFER_SIZE: u64 = size_of::<f32>() as u64;
        let matrix: [[f32; 4]; 3] = read_f32_vec(vp, self.bone_pointer + self.hitbox.head.index as u64 * (12 * BUFFER_SIZE), 12)
            .chunks_exact(4)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<[f32; 4]>>()
            .try_into()
            .unwrap();
        println!("addr -> {}", self.bone_pointer + self.hitbox.head.index as u64 * (12 * BUFFER_SIZE));
        self.hitbox.head.position = [matrix[0][3] + vec_abs_origin[0], matrix[1][3] + vec_abs_origin[1], matrix[2][3] + vec_abs_origin[2]];
    }
}


#[derive(Debug, Copy, Clone, Default)]
pub struct LocalPlayer {
    pub base: u64,
    pub render_pointer: u64,
    pub matrix_pointer: u64,
    pub view_matrix: [[f32; 4]; 4],
}

impl LocalPlayer {
    pub fn update_pointer(&mut self, vp: VmmProcess) {
        self.render_pointer = read_u64(vp, self.base + VIEW_RENDER);
        self.matrix_pointer = read_u64(vp, self.render_pointer + VIEW_MATRIX);
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


