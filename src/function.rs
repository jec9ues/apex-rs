use std::fmt::Error;
use crate::mem::*;
use crate::constants::offsets::*;
use log4rs;
use std::mem::size_of;
use std::pin::Pin;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::u8;
use std::default::Default;
use log::debug;
use memprocfs::*;
use pretty_hex::*;

use mouse_rs::{types::keys::Keys, Mouse};
use crate::data::{Bone, Player};
use crate::egui_overlay::egui::Pos2;




fn move_and_press() {
    let mouse = Mouse::new();
    mouse.move_to(500, 500).expect("Unable to move mouse");
    mouse.press(&Keys::RIGHT).expect("Unable to press button");
    mouse.release(&Keys::RIGHT).expect("Unable to release button");
}



/// addr -> entity_list address
/*pub fn get_player_pointer(vp: VmmProcess, addr: u64) -> Vec<u64> {
    const SIZE: usize = (60 << 5);
    // add (1 << 5) skip CWorld
    let data = read_mem(vp, addr + (1 << 5), SIZE);
    let mut array: Vec<u64> = Vec::new();
    // entity_list chunk size
    let chunk_size = 32;
    // save chunk
    let chunks: Vec<&[u8]> = data.chunks_exact(chunk_size).collect();

    for chunk in chunks.into_iter() {
        // get u64 entity pointer
        let chunk_u64 = u64::from_le_bytes(chunk[..8].try_into().expect("Chunk has unexpected length"));
        if chunk_u64 != 0 { array.push(chunk_u64) };

        // println!("num: {i} ptr -> {:x}", ptr);
        //let enable = read_u64(vmmprocess, (ptr + GLOW_ENABLE) as u64);
        // if enable == 7 {
        //     continue
        // }
        // write_u8(vmmprocess, ptr + GLOW_ENABLE, 1);
        // write_u8(vmmprocess, ptr+ GLOW_THROUGH_WALL, 1);
        // write_mem(vmmprocess, ptr + GLOW_TYPE, [101, 102, 96, 75].to_vec());

        // info!("ce -> {:?}", read_mem(vmmprocess, (ptr + GLOW_ENABLE) as u64, 32).hex_dump());
        // info!("ct -> {:?}", read_mem(vmmprocess, (ptr + GLOW_TYPE) as u64, 32).hex_dump());
        // info!("cw -> {:?}", read_mem(vmmprocess, (ptr + GLOW_THROUGH_WALL) as u64, 32).hex_dump());
        // println!("{:?}",  read_u64(vmmprocess, (ptr + GLOW_TYPE + 0x4) as u64));
        // write_mem(vmmprocess, (ptr + GLOW_TYPE) as u64, [67, 67, 9f, 41].to_vec());
        // write_f32(vmmprocess, ptr + GLOW_COLOR, 10.0);        // r
        // write_f32(vmmprocess, ptr + GLOW_COLOR + 0x4, 10.0); // g
        // write_f32(vmmprocess, ptr + GLOW_COLOR + 0x8, 1.0);  // b

        //
        // write_f32(vmmprocess, (ptr + GLOW_DISTANCE) as u64, 40000.0);
    }
    return array;
}*/


/*pub fn get_entity_pointer(vp: VmmProcess, addr: u64) -> Vec<u64> {
    const SIZE: usize = (15000 << 5);
    // add (1 << 5) skip CWorld
    let data = read_mem(vp, addr + (1 << 5), SIZE);
    let mut array: Vec<u64> = Vec::new();
    // entity_list chunk size
    let chunk_size = 32;
    // save chunk
    let chunks: Vec<&[u8]> = data.chunks_exact(chunk_size).collect();

    for chunk in chunks.iter() {
        // get u64 entity pointer
        let chunk_u64 = u64::from_le_bytes(chunk[..8].try_into().expect("Chunk has unexpected length"));
        if chunk_u64 != 0 { array.push(chunk_u64) };
    }
    return array;
}*/

/// addr -> entity_list address
pub fn get_player_pointer(vp: VmmProcess, addr: u64) -> Vec<u64> {
    const SIZE: usize = (60 << 5);
    // add (1 << 5) skip CWorld
    let data = read_mem(vp, addr + (1 << 5), SIZE);

    data.chunks_exact(32)
        .map(|chunk| u64::from_le_bytes(chunk[..8].try_into().unwrap()))
        .filter(|&chunk_u64| chunk_u64 != 0)
        .collect()
}

/// addr -> entity_list address
pub fn get_entity_pointer(vp: VmmProcess, addr: u64) -> Vec<u64> {
    const SIZE: usize = (15000 << 5);
    // add (1 << 5) skip CWorld
    let data = read_mem(vp, addr + (1 << 5), SIZE);

    data.chunks_exact(32)
        .map(|chunk| u64::from_le_bytes(chunk[..8].try_into().unwrap()))
        .filter(|&chunk_u64| chunk_u64 != 0)
        .collect()
}


pub fn item_glow(vp: VmmProcess, addr: u64) {
    let entity_list: u64 = addr + CL_ENTITYLIST;
    let end = entity_list + (15000 << 5);
    let area = (15000 << 5);

    let data = read_mem(vp, entity_list, area );
    let mut array: [u64; 15000] = [0; 15000];
    println!("start -> {:x} end -> {:x}", entity_list, end);
// 定义块的大小（32个字节）
    let chunk_size = 32;

// 使用 chunks_exact 方法将 data 切成子数组块
    let chunks: Vec<&[u8]> = data.chunks_exact(chunk_size).collect();
    // 创建一个向量用于存储满足条件的 chunk_u64 值

// 现在 chunks 中包含了每个大小为 chunk_size 的子数组
    for (i, chunk) in chunks.iter().enumerate() {

        // 取每组的前8个字节
        let chunk_u64 = u64::from_le_bytes(chunk[..8].try_into().expect("Chunk has unexpected length"));
        if chunk_u64 == 0 { continue; }
        array[i] = chunk_u64;
        // let name = read_string(vp, chunk_u64 + SIGN_NAME as u64);
        // println!("num -> {} chunk_64 -> {:x} name -> {}", i, chunk_u64, name);
        // info!("{}", read_u64(vp, chunk_u64 + 0x16A0));
        // info!("{:?}", read_mem(vp, chunk_u64 + 0x16A0, 32).hex_dump());
        let team_num = read_u64(vp, (chunk_u64 + TEAM_NUM ) as u64);
        if team_num == 97 {
            let health = read_u64(vp, (chunk_u64 + HEALTH ) as u64);

            if health == 0 { continue; };
            // info!("health -> {}", health);
            println!("chunk_64 -> {:x}", chunk_u64);
            // println!("{:?}", read_mem(vp, (chunk_u64 + GLOW_TYPE) as u64, 16).hex_dump());
            write_u8(vp, (chunk_u64 + GLOW_THROUGH_WALL ) as u64, 1);
            write_mem(vp, (chunk_u64 + GLOW_TYPE ) as u64, [101, 102, 96, 75].to_vec());
        }
    }
    // println!("{:?}", array);
    // write_u64(vp, (chunk_u64 + 0x02f0) as u64, 1363184265); // loba-style m_highlightFunctionBits
}

pub fn im_player_glow(vp: VmmProcess, addr: u64) {
    let entity_list: u64 = addr + CL_ENTITYLIST;
    let end = entity_list + (60 << 5);
    let area = (60 << 5);

    let data = read_mem(vp, entity_list, area );
    let mut array: Vec<u64> = Vec::new();
    println!("start -> {:x} end -> {:x}", entity_list, end);
// 定义块的大小（32个字节）
    let chunk_size = 32;

// 使用 chunks_exact 方法将 data 切成子数组块
    let chunks: Vec<&[u8]> = data.chunks_exact(chunk_size).collect();
    // 创建一个向量用于存储满足条件的 chunk_u64 值

// 现在 chunks 中包含了每个大小为 chunk_size 的子数组
    for (i, chunk) in chunks.iter().enumerate() {

        // 取每组的前8个字节
        let chunk_u64 = u64::from_le_bytes(chunk[..8].try_into().expect("Chunk has unexpected length"));
        if chunk_u64 == 0 { continue; }
        println!("{:x}", chunk_u64);
        array.push(chunk_u64);
        // let name = read_string(vp, chunk_u64 + SIGN_NAME as u64);
        // println!("num -> {} chunk_64 -> {:x} name -> {}", i, chunk_u64, name);
        // info!("{}", read_u64(vp, chunk_u64 + 0x16A0));
        // info!("{:?}", read_mem(vp, chunk_u64 + 0x16A0, 32).hex_dump());
    }
}


/*    let team_num = read_u64(vp, (chunk_u64 + TEAM_NUM) as u64);
    if team_num == 97 {

        let health = read_u64(vp, (chunk_u64 + HEALTH) as u64);

        if health == 0 { continue };
        // info!("health -> {}", health);
        println!("chunk_64 -> {:x}", chunk_u64);
        // println!("{:?}", read_mem(vp, (chunk_u64 + GLOW_TYPE) as u64, 16).hex_dump());
        write_u8(vp, (chunk_u64 + GLOW_THROUGH_WALL) as u64, 1);
        write_mem(vp, (chunk_u64 + GLOW_TYPE) as u64, [101, 102, 96, 75].to_vec());

    }*/


pub fn get_client_class_name(vp: VmmProcess, ptr: u64) -> String {
    let client_networkable_vtable = read_u64(vp, ptr + 3 * 8);
    let get_client_entity = read_u64(vp, client_networkable_vtable + 3 * 8);
    let offset = read_u32(vp, get_client_entity + 3);
    let network_name_ptr = read_u64(vp, get_client_entity + offset as u64 + 7 + 16);
    let network_name = read_string(vp, network_name_ptr);
    return network_name;
}


pub fn item_loba_glow(vp: VmmProcess, base: u64) {
    let addr = base + CL_ENTITYLIST;
    let pointer_list = get_entity_pointer(vp, addr);
    for ent in pointer_list.into_iter() {
        let name = get_client_class_name(vp, ent);
        debug!("network_name -> {}", name);
        if name == "CPropSurvival" {
            write_u64(vp, ent + 0x02f0, 1363184265); // loba-style
        }
    }
}

// println!("item id -> {}", item_id.unwrap());
/*        let team = read_u64(vp, ptr + TEAM_NUM );
        if team.unwrap() == 97 {

            write_u64(vp, ptr + GLOW_THROUGH_WALL , 2);
            write_mem(vp, ptr + GLOW_TYPE , [101, 102, 96, 75].to_vec());
            write_f32(vp, ptr + GLOW_COLOR , 0.0);
            write_f32(vp, ptr + GLOW_COLOR + 0x4, 1.0);
            write_f32(vp, ptr + GLOW_COLOR + 0x8, 0.0);
        }
        write_u64(vp, ptr + 0x02f0, 1363184265); // loba-style*/
/*pub fn player_glow( vp: VmmProcess, addr: u64) {
    for i in 0..61 {
        let addr: u64 = addr + CL_ENTITYLIST + (i << 5);
        if let Ok(ptr) = read_u64(vp, addr) {
            // null ptr
            if ptr == 0 {
                continue;
            }
            // dead
            if let Ok(dead) = read_u64(vp, addr + LIFE_STATE ) {
                if !(dead > 0) {
                    continue;
                }
            }
            const SIZE: usize = size_of::<f64>();
            //entity pos x, y, z
            let lx = read_f32(vp, (ptr + LOCAL_ORIGIN ));
            let ly = read_f32(vp, (ptr + LOCAL_ORIGIN + SIZE));
            let lz = read_f32(vp, (ptr + LOCAL_ORIGIN + SIZE + SIZE));
            println!("x -> {} y -> {} z -> {}", lx.unwrap(), ly.unwrap(), lz.unwrap());
            if lx.unwrap() == 0.0 { continue; };
            let w2s = world_to_screen(get_matrix(0x140000000, vp), Pos3 { x: lx.unwrap(), y: ly.unwrap(), z: lz.unwrap() }, Pos2::new(2560.0, 1440.0));
            println!("{:?}", w2s);
            // let bonePtr = read_u64(vp, ptr + BONE );
            // let bone = readBoneFromHitBox(vp, ptr , 0);
            // println!("bone -> {}", bone);
            let headPosition = readBonePosition(vp, ptr , 0);
            let h2s = world_to_screen(getMatrix(0x140000000, vp), Pos3 { x: headPosition[0], y: headPosition[1], z: headPosition[2] }, Pos2::new(2560.0, 1440.0));
            // println!("ptr: {:?}",  h2s);
            write_u64(vp, ptr + GLOW_ENABLE , 1);
            write_u64(vp, ptr + GLOW_THROUGH_WALL , 1);
            write_mem(vp, ptr + GLOW_TYPE , [101, 102, 96, 75].to_vec());
            write_f32(vp, ptr + GLOW_COLOR , 0.0);
            write_f32(vp, ptr + GLOW_COLOR + 0x4, 1.0);
            write_f32(vp, ptr + GLOW_COLOR + 0x8, 10.0);
        }
    }
}*/

pub fn player_head(vp: VmmProcess, base: u64) -> Vec<Pos2> {
    let mut Vh2s: Vec<Pos2> = Vec::new();
    for i in 1..61 {
        let addr: u64 = base + CL_ENTITYLIST + (i << 5);
        let ptr = read_u64(vp, addr);
        {
            // null ptr
            if ptr == 0 {
                continue;
            }
            // let string1 = String::from("CPlayer");
            // let string2 = String::from("CAI_BaseNPC");
            // let name = get_client_class_name(vp, ptr);
            // if name != string1 && name != string2 {
            //     println!("{}", name);
            //     continue;
            // }

            // dead
            let dead = read_u64(vp, addr + LIFE_STATE );
            {
                if !(dead > 0) {
                    continue;
                }
            }
            for i in 0..1 {
                let head_position = read_bone_position(vp, ptr , 0);
                // println!("{:?}", head_position);
                if head_position[0] == 1.5 {
                    continue;
                }
                let h2s = world_to_screen(get_matrix(vp, base), Pos3 { x: head_position[0], y: head_position[1], z: head_position[2] }, Pos2::new(2560.0, 1440.0));
                // println!("{:?}", h2s);
                if h2s.x != 0.0 {
                    Vh2s.push(h2s);
                };
            }
        }
    }
    return Vh2s;
}

pub fn player_bone(vp: VmmProcess, base: u64) -> Vec<Pos2> {
    let mut Vh2s: Vec<Pos2> = Vec::new();
    let player_pointer = get_player_pointer(vp, base + CL_ENTITYLIST);
    for ptr in player_pointer {
        let mut da = Player { pointer: ptr, ..Default::default() };
        da.update_bone_index(vp);
        da.update_bone_position(vp);
        let h2s = world_to_screen(get_matrix(vp, base), Pos3 { x: da.hitbox.head.position[0], y: da.hitbox.head.position[1], z: da.hitbox.head.position[2] }, Pos2::new(2560.0, 1440.0));
        Vh2s.push(h2s);
    }
    return Vh2s;
}

pub fn player_bone_position(vp: VmmProcess, base: u64) -> Vec<Pos2> {
    let mut Vh2s: Vec<Pos2> = Vec::new();
    let player_pointer = get_player_pointer(vp, base + CL_ENTITYLIST);
    for ptr in player_pointer {

        let head_position = read_bone_position(vp, ptr , 0);
        // println!("{:?}", head_position);
        if head_position[0] == 1.5 {
            continue;
        }
        let h2s = world_to_screen(get_matrix(vp, base), Pos3 { x: head_position[0], y: head_position[1], z: head_position[2] }, Pos2::new(2560.0, 1440.0));
        // println!("{:?}", h2s);
        if h2s.x != 0.0 {
            Vh2s.push(h2s);
        };

    }
    return Vh2s;
}

pub fn cache_player_bone(vp: VmmProcess, base: u64) -> Vec<Pos2> {
    let mut Vh2s: Vec<Pos2> = Vec::new();
    for i in 0..61 {
        let addr: u64 = base + CL_ENTITYLIST + (i << 5);
        let ptr = read_u64(vp, addr);
        {
            // null ptr
            if ptr == 0 {
                continue;
            }
            // let string1 = String::from("CPlayer");
            // let string2 = String::from("CAI_BaseNPC");
            // let name = get_client_class_name(vp, ptr);
            // if name != string1 && name != string2 {
            //     println!("{}", name);
            //     continue;
            // }

            // dead
            let dead = read_u64(vp, addr + LIFE_STATE );
            {
                if !(dead > 0) {
                    continue;
                }
            }

            for i in 0..1 {
                let head_position = read_bone_position(vp, ptr , 0);
                // println!("{:?}", head_position);
                if head_position[0] == 1.5 {
                    continue;
                }
                let h2s = world_to_screen(get_matrix(vp, base), Pos3 { x: head_position[0], y: head_position[1], z: head_position[2] }, Pos2::new(2560.0, 1440.0));
                // println!("{:?}", h2s);
                if h2s.x != 0.0 {
                    Vh2s.push(h2s);
                };
            }
        }
    }
    return Vh2s;
}
/// addr -> local_player address
pub fn weaponx_entity(vp: VmmProcess, addr: u64, base: u64) {
    let local_player_pointer = read_u64(vp, addr);
    let mut weapon_handle = read_u64(vp, local_player_pointer + WEAPON);
    weapon_handle &= 0xffff;

    let weapon_entity = read_u64(vp, base + CL_ENTITYLIST + (weapon_handle << 5));
    let index = read_u16(vp, weapon_entity + WEAPON_NAME);
    let projectile_speed = read_f32(vp, weapon_entity + BULLET_SPEED);
    let projectile_scale = read_f32(vp, weapon_entity + BULLET_SCALE);
    let zoom_fov = read_f32(vp, weapon_entity + ZOOM_FOV);
    let ammo = read_u16(vp, weapon_entity + AMMO);
    let semi_auto = read_u16(vp, weapon_entity + SEMI_AUTO);

    println!("wephandle -> {:x}", weapon_handle);
    println!("wep_entity -> {:x}", weapon_entity);
    println!("index -> {:?}", index);
    println!("projectile_speed -> {:?}", projectile_speed);
    println!("projectile_scale -> {:?}", projectile_scale);
    println!("zoom_fov -> {:?}", zoom_fov);
    println!("ammo -> {:?}", ammo);
    println!("semi_auto -> {:?}", semi_auto);
}

/// addr -> base address
pub fn get_matrix(vp: VmmProcess, addr: u64) -> [[f32; 4]; 4] {
    let render_pointer = read_u64(vp, addr + VIEW_RENDER);
    let matrix_pointer = read_u64(vp, render_pointer + VIEW_MATRIX);

    let mut matrix: [[f32; 4]; 4] = [[0.0; 4]; 4];
    const BUFFER_SIZE: u64 = size_of::<f32>() as u64;
    let mut x = 0;
    for i in 0..4 {
        for j in 0..4 {
            let view_matrix = read_f32(vp, matrix_pointer + x * BUFFER_SIZE);
            matrix[i][j] = view_matrix;
            // println!("{} {} {} {:x}", i, j, r2, off);
            x += 1;
        };
    };
    // 将 Vec<f32> 转换为固定长度的二维数组 (4x4)
    // println!("{:?}", matrix);
    return matrix;
}
/// addr -> base address
pub fn get_matrix_test(vp: VmmProcess, addr: u64) -> [[f32; 4]; 4] {
    let render_pointer = read_u64(vp, addr + VIEW_RENDER);
    let matrix_pointer = read_u64(vp, render_pointer + VIEW_MATRIX);

    let matrix: [[f32; 4]; 4] = read_f32_vec(vp, matrix_pointer, 16)
        .chunks_exact(4)
        .map(|chunk| chunk.try_into().unwrap())
        .collect::<Vec<[f32; 4]>>()
        .try_into()
        .unwrap();
    // 将 Vec<f32> 转换为固定长度的二维数组 (4x4)
    // println!("{:?}", matrix);
    return matrix;
}

pub fn get_bone_matrix(vp: VmmProcess, addr: u64) -> [[f32; 4]; 3] {
    // println!("{:?}", read_mem(vp, addr, 200).hex_dump());
    let mut matrix: [[f32; 4]; 3] = [[0.0; 4]; 3];
    let mut x: u64 = 0;
    const SIZE: u64 = size_of::<f32>() as u64;
    for i in 0..3 {
        for j in 0..4 {
            let view_matrix = read_f32(vp, addr + x * SIZE);
            matrix[i][j] = view_matrix;
            x += 1;
        }
    }
    // println!("{:?}", matrix);
    return matrix;
}



pub struct Pos3 {
    /// How far to the right.
    pub x: f32,

    /// How far down.
    pub y: f32,
    // implicit w = 1
    pub z: f32,
}

/*pub fn world_to_screen(matrix: [[f32; 4]; 4], vector: Pos3, screen_size: Pos2) -> Pos2 {
    let mut transformed = Pos3 { x: 0.0, y: 0.0, z: 0.0 };
    transformed.x = vector.y * matrix[0][1] + vector.x * matrix[0][0] + vector.z * matrix[0][2] + matrix[0][3];
    transformed.y = vector.y * matrix[1][1] + vector.x * matrix[1][0] + vector.z * matrix[1][2] + matrix[1][3];
    transformed.z = vector.y * matrix[3][1] + vector.x * matrix[3][0] + vector.z * matrix[3][2] + matrix[3][3];
    let mut res = Pos2::new(0.0, 0.0);
    if (transformed.z < 0.001) {
        return res;
    }

    transformed.x *= 1.0 / transformed.z;
    transformed.y *= 1.0 / transformed.z;

    let halfResoltion = Pos2::new(screen_size.x / 2.0, screen_size.y / 2.0);

    res.x = halfResoltion.x + transformed.x * halfResoltion.x;
    res.y = halfResoltion.y - transformed.y * halfResoltion.y;
    // println!("transformed.x -> {}", transformed.x);
    // println!("transformed.y -> {}", transformed.y);
    // println!("transformed.z -> {}", transformed.z);
    // println!("res.x -> {}", res.x);
    // println!("res.y -> {}", res.y);
    return res;
}*/

pub fn world_to_screen(matrix: [[f32; 4]; 4], vector: Pos3, screen_size: Pos2) -> Pos2 {
    let transformed = [
        vector.x * matrix[0][0] + vector.y * matrix[0][1] + vector.z * matrix[0][2] + matrix[0][3],
        vector.x * matrix[1][0] + vector.y * matrix[1][1] + vector.z * matrix[1][2] + matrix[1][3],
        vector.x * matrix[2][0] + vector.y * matrix[2][1] + vector.z * matrix[2][2] + matrix[2][3],
        vector.x * matrix[3][0] + vector.y * matrix[3][1] + vector.z * matrix[3][2] + matrix[3][3],
    ];

    if transformed[3] < 0.001 {
        return Pos2::new(0.0, 0.0);
    }

    let inv_w = 1.0 / transformed[3];
    let x = transformed[0] * inv_w;
    let y = transformed[1] * inv_w;

    let half_resolution = Pos2::new(screen_size.x / 2.0, screen_size.y / 2.0);

    Pos2::new(half_resolution.x + x * half_resolution.x, half_resolution.y - y * half_resolution.y)
}

pub fn is_valid_pointer(pointer: u64) -> bool {
    return pointer > 0x00010000 && pointer < 0x7FFFFFFEFFFF;
}
///addr -> entity pointer
pub fn read_bone_from_hitbox(vp: VmmProcess, addr: u64, hitbox: u64) -> u16 {
    let model_pointer = read_u64(vp, addr + STUDIOHDR);
    if !is_valid_pointer(model_pointer) {
        return u16::MAX;
    }

    let studio_hdr = read_u64(vp, model_pointer + 0x8);
    if !is_valid_pointer(studio_hdr + 0x34) {
        return u16::MAX;
    }
    // println!("modelPtr -> {:x}", modelPtr.unwrap());
    // println!("studioPtr -> {:x}", studio_hdr.unwrap());
    let hitbox_cache = read_u16(vp, studio_hdr + 0x34) as u64;
    let hitbox_array = studio_hdr + ((hitbox_cache & 0xFFFE) << (4 * (hitbox_cache & 1)));
    if !is_valid_pointer(hitbox_array + 0x4) {
        return u16::MAX;
    }
    // println!("hitboxCache -> {:x}", p3);
    // println!("hitboxArray -> {:x}", hitboxArray);
    let index_cache = read_u16(vp, hitbox_array + 0x4);
    let hitbox_index = ((index_cache & 0xFFFE) << (4 * (index_cache & 1)));
    // println!("indexCache -> {:x}", p4);
    // println!("hitboxIndex -> {:x}", hitboxIndex);
    let bone_pointer = hitbox_index as u64 + hitbox_array + (hitbox * 0x20);
    if !is_valid_pointer(bone_pointer) {
        return u16::MAX;
    }
    // println!("bonePtr -> {:x}", bone_pointer);
    let res = read_u16(vp, bone_pointer);
    return res;
}

pub fn read_bone_position(vp: VmmProcess, addr: u64, hitbox: u64) -> [f32; 3] {
    const offset: [f32; 3] = [1.5, 0.0, 0.0];
    const BUFFER_SIZE: u64 = size_of::<f32>() as u64;
    let vecAbsOrigin1 = read_f32(vp, addr + ABS_VECTORORIGIN);
    let vecAbsOrigin2 = read_f32(vp, addr + ABS_VECTORORIGIN + BUFFER_SIZE);
    let vecAbsOrigin3 = read_f32(vp, addr + ABS_VECTORORIGIN + BUFFER_SIZE + BUFFER_SIZE);
    let _vecAbsOrigin: [f32; 3] = [vecAbsOrigin1, vecAbsOrigin2, vecAbsOrigin3];

    let bone = read_bone_from_hitbox(vp, addr, hitbox) as u64;

    if bone < 0 || bone > 255 {
        return [_vecAbsOrigin[0] + offset[0], _vecAbsOrigin[1] + offset[1], _vecAbsOrigin[2] + offset[2]];
    }

    let bone_pointer = read_u64(vp, addr + BONE);
    if !is_valid_pointer(bone_pointer) {
        println!("Invalid bone pointer ");
        return [_vecAbsOrigin[0] + offset[0], _vecAbsOrigin[1] + offset[1], _vecAbsOrigin[2] + offset[2]];
    }
    // println!("bone -> {:x}", bone_pointer);
    // let matrix = get_bone_matrix(vp, bone_pointer + bone * (12 * BUFFER_SIZE));
    let matrix: [[f32; 4]; 3] = read_f32_vec(vp, bone_pointer + bone * (12 * BUFFER_SIZE), 12)
        .chunks_exact(4)
        .map(|chunk| chunk.try_into().unwrap())
        .collect::<Vec<[f32; 4]>>()
        .try_into()
        .unwrap();


    let bone_position = [matrix[0][3], matrix[1][3], matrix[2][3]];

    if !is_valid(bone_position) {
        return [_vecAbsOrigin[0] + offset[0], _vecAbsOrigin[1] + offset[1], _vecAbsOrigin[2] + offset[2]];
    }

    let res = [bone_position[0] + _vecAbsOrigin[0], bone_position[1] + _vecAbsOrigin[1], bone_position[2] + _vecAbsOrigin[2]];

    return res;
}

pub fn read_bone(vp: VmmProcess, addr: u64, bone_index: u16) -> [f32; 3] {
    const offset: [f32; 3] = [1.5, 0.0, 0.0];
    const BUFFER_SIZE: u64 = size_of::<f32>() as u64;
    let _vecAbsOrigin: [f32; 3] = read_f32_vec(vp, addr + ABS_VECTORORIGIN, 3).as_slice().try_into().unwrap();

    if bone_index < 0 || bone_index > 255 {
        return [_vecAbsOrigin[0] + offset[0], _vecAbsOrigin[1] + offset[1], _vecAbsOrigin[2] + offset[2]];
    }
    println!("bone -> {}", bone_index);
    let bone_pointer = read_u64(vp, addr + BONE);
    if !is_valid_pointer(bone_pointer) {
        println!("Invalid bone pointer ");
        return [_vecAbsOrigin[0] + offset[0], _vecAbsOrigin[1] + offset[1], _vecAbsOrigin[2] + offset[2]];
    }

    let matrix: [[f32; 4]; 3] = read_f32_vec(vp, bone_pointer + bone_index as u64 * (12 * BUFFER_SIZE), 12)
        .chunks_exact(4)
        .map(|chunk| chunk.try_into().unwrap())
        .collect::<Vec<[f32; 4]>>()
        .try_into()
        .unwrap();

    let bone_position = [matrix[0][3], matrix[1][3], matrix[2][3]];

    if !is_valid(bone_position) {
        return [_vecAbsOrigin[0] + offset[0], _vecAbsOrigin[1] + offset[1], _vecAbsOrigin[2] + offset[2]];
    }

    let res = [bone_position[0] + _vecAbsOrigin[0], bone_position[1] + _vecAbsOrigin[1], bone_position[2] + _vecAbsOrigin[2]];

    return res;
}

pub fn is_valid(p: [f32; 3]) -> bool {
    !(p[0].is_nan() || p[0].is_infinite() || p[1].is_nan() || p[1].is_infinite() || p[2].is_nan() || p[2].is_infinite())
}

