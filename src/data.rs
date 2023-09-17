use std::mem::size_of;
use std::ptr::addr_of;
use std::thread::sleep;
use memprocfs::*;
use crate::constants::offsets::*;
use crate::function::{Pos3, read_bone_from_hitbox};
use crate::mem::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Player {
    pub pointer: u64,
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

        self.hitbox.head.index = read_u16(vp, hitbox_index as u64 + hitbox_array + (0 * 0x20));
    }

    pub fn updaye_bone_position(&mut self, vp: VmmProcess) {
        let vec_abs_origin: [f32; 3] = read_f32_vec(vp, self.pointer + ABS_VECTORORIGIN, 3).as_slice().try_into().unwrap();
        let bone_pointer = read_u64(vp, self.pointer + BONE);
        const BUFFER_SIZE: u64 = size_of::<f32>() as u64;
        let matrix: [[f32; 4]; 3] = read_f32_vec(vp, bone_pointer + self.hitbox.head.index as u64 * (12 * BUFFER_SIZE), 12)
            .chunks_exact(4)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<[f32; 4]>>()
            .try_into()
            .unwrap();

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


