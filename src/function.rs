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
use egui_backend::egui::Color32;
use egui_backend::egui::CursorIcon::Text;
use log::{debug, info};
use memprocfs::*;
use pretty_hex::*;

use mouse_rs::{types::keys::Keys, Mouse};
use crate::data::{Bone, Player, Pos3};
use crate::egui_overlay::egui::Pos2;


fn move_and_press() {
    let mouse = Mouse::new();
    mouse.move_to(500, 500).expect("Unable to move mouse");
    mouse.press(&Keys::RIGHT).expect("Unable to press button");
    mouse.release(&Keys::RIGHT).expect("Unable to release button");
}




/// addr -> entity_list address
pub fn get_player_pointer(vp: VmmProcess, addr: u64) -> Vec<u64> {
    const SIZE: usize = (60 << 5);
    // add (1 << 5) skip CWorld
    let data = read_mem(vp, addr + (1 << 5), SIZE);

    data.chunks_exact(0x20)
        .map(|chunk| u64::from_le_bytes(chunk[..8].try_into().unwrap()))
        .filter(|&chunk_u64| chunk_u64 != 0)
        .collect()
}

pub fn get_player_pointer_index(vp: VmmProcess, addr: u64) -> Vec<[u64; 2]> {
    const SIZE: usize = (60 << 5);
    // add (1 << 5) skip CWorld
    let data = read_mem(vp, addr, SIZE);

    data.chunks_exact(0x20)
        .enumerate()
        .filter_map(|(index, chunk)| {
            let chunk_u64 = u64::from_le_bytes(chunk[..8].try_into().unwrap());
            if index == 0 {
                None
            }
            else if chunk_u64 != 0 {
                // println!("Index: {}, Value: {}", index, chunk_u64);
                Some([index as u64, chunk_u64])
            } else {
                // println!("Index: {}, Value: {}", index, chunk_u64);
                None
            }
        })
        .collect()
}



/// addr -> entity_list address
pub fn get_entity_pointer(vp: VmmProcess, addr: u64) -> Vec<u64> {
    const SIZE: usize = (15000 << 5);
    // add (1 << 5) skip CWorld
    let data = read_mem(vp, addr + (1 << 5), SIZE);

    data.chunks_exact(0x20)
        .map(|chunk| u64::from_le_bytes(chunk[..8].try_into().unwrap()))
        .filter(|&chunk_u64| chunk_u64 != 0)
        .collect()
}


pub fn item_glow(vp: VmmProcess, addr: u64) {
    let entity_list: u64 = addr + CL_ENTITYLIST;
    let end = entity_list + (15000 << 5);
    let area = (15000 << 5);

    let data = read_mem(vp, entity_list, area);
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
        let team_num = read_u64(vp, (chunk_u64 + TEAM_NUM) as u64);
        if team_num == 97 {
            let health = read_u64(vp, (chunk_u64 + HEALTH) as u64);

            if health == 0 { continue; };
            // info!("health -> {}", health);
            println!("chunk_64 -> {:x}", chunk_u64);
            // println!("{:?}", read_mem(vp, (chunk_u64 + GLOW_TYPE) as u64, 16).hex_dump());
            write_u8(vp, (chunk_u64 + GLOW_THROUGH_WALL) as u64, 1);
            write_mem(vp, (chunk_u64 + GLOW_TYPE) as u64, [101, 102, 96, 75].to_vec());
        }
    }
    // println!("{:?}", array);
    // write_u64(vp, (chunk_u64 + 0x02f0) as u64, 1363184265); // loba-style m_highlightFunctionBits
}
/// addr -> base address
pub fn im_player_glow(vp: VmmProcess, addr: u64, x: u16) {
    let entity_list: u64 = addr + CL_ENTITYLIST;
    let end = entity_list + (15000 << 5);
    let area = (15000 << 5);

    let data = read_mem(vp, entity_list, area);
    let mut array: Vec<u64> = Vec::new();
    // println!("start -> {:x} end -> {:x}", entity_list, end);
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
        array.push(chunk_u64);
        let name = get_client_class_name(vp, chunk_u64);
        // println!("{name}");
/*        if name == "CPropSurvival" {// 12 13 22 25 45 47 51 65 129 132 133 145 149 156 170 174 179 191?
            // println!("in");
            write_u8(vp, chunk_u64 + GLOW_THROUGH_WALL, 1);
            write_u8(vp, chunk_u64 + 0x270, 0);
            write_u8(vp, chunk_u64 + GLOW_ENABLE, 0);
            write_u8(vp, chunk_u64 + GLOW_ENABLE + 0x4, x);
        }*/

        let local_ptr = read_u64(vp, addr + LOCAL_PLAYER);

        write_u16(vp, chunk_u64 + TEAM_NUM, read_u16(vp, local_ptr + TEAM_NUM));
        // sleep(Duration::from_secs(1));





        // write_f32(vp, chunk_u64 + GLOW_COLOR + 0x40 , 0.0);
        // write_f32(vp, chunk_u64 + GLOW_COLOR + 0x40 + 0x4, 20.0);
        // write_f32(vp, chunk_u64 + GLOW_COLOR + 0x40 + 0x8, 10.0);

        // write_mem(vp, chunk_u64 + GLOW_TYPE, [101, 102, 96, 75].to_vec());
        // info!("BOT -> {:?}", read_mem(vp, chunk_u64 + GLOW_COLOR + 0x40 , 0x50).hex_dump());
        // let team_num = read_u64(vp, (chunk_u64 + TEAM_NUM) as u64);
        if false {
            write_u32(vp, chunk_u64 + OFFSET_HIGHLIGHTCURRENTCONTEXTID, 0);  // context id to 1
            write_u32(vp, chunk_u64 + OFFSET_HIGHLIGHTVISIBILITYTYPE, 2); // visibility to always
            write_u8(vp, chunk_u64 + OFFSET_HIGHLIGHTSERVERACTIVESTATES, 200);  // maybe a rarely used settings

            let highlightSettingsPtr = read_u64(vp, addr + OFFSET_HIGHLIGHTSETTINGS);
            // println!("highlightSettingsPtr -> {:x}", highlightSettingsPtr);
            write_mem(vp, highlightSettingsPtr + 40 * 200 + 4, [137, 138, 70, 64].to_vec());
            write_f32(vp, highlightSettingsPtr + 40 * 200 + 8, 0.0);
            write_f32(vp, highlightSettingsPtr + 40 * 200 + 8 + 0x4, 0.1);
            write_f32(vp, highlightSettingsPtr + 40 * 200 + 8 + 0x8, 0.1);
            // write_u8(vp, chunk_u64 + GLOW_ENABLE, 7);
            // write_u8(vp, chunk_u64 + GLOW_THROUGH_WALL, 2);
            // write_mem(vp, chunk_u64 + GLOW_TYPE, [101, 102, 96, 75].to_vec());

            // let name = read_string(vp, chunk_u64 + SIGN_NAME as u64);
            // println!("num -> {} chunk_64 -> {:x} name -> {}", i, chunk_u64, name);
            // info!("{}", read_u64(vp, chunk_u64 + 0x16A0));

            //
        }
    }
}

pub const TEAM_COLOR: [Color32; 23] = [
    Color32::DARK_GRAY,
    Color32::from_rgb(20, 150, 0),
    Color32::from_rgb(120, 50, 0),
    Color32::from_rgb(20, 150, 50),
    Color32::from_rgb(0, 50, 50),
    Color32::LIGHT_GRAY,
    Color32::BROWN,
    Color32::DARK_RED,
    Color32::RED,
    Color32::from_rgb(200, 0, 0),
    Color32::LIGHT_RED,
    Color32::YELLOW,
    Color32::LIGHT_YELLOW,
    Color32::KHAKI,
    Color32::DARK_GREEN,
    Color32::GREEN,
    Color32::from_rgb(0, 200, 0),
    Color32::LIGHT_GREEN,
    Color32::DARK_BLUE,
    Color32::BLUE,
    Color32::LIGHT_BLUE,
    Color32::GOLD,
    Color32::from_rgb(0, 0, 200),
];



pub fn get_client_class_name(vp: VmmProcess, ptr: u64) -> String {
    let client_networkable_vtable = read_u64(vp, ptr + 3 * 8);
    let get_client_entity = read_u64(vp, client_networkable_vtable + 3 * 8);
    let offset = read_u32(vp, get_client_entity + 3);
    let network_name_ptr = read_u64(vp, get_client_entity + offset as u64 + 7 + 16);
    let network_name = read_string(vp, network_name_ptr);
    // println!("{}", network_name);
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



/// addr -> local_player address
pub fn weaponx_entity(vp: VmmProcess, addr: u64, base: u64) -> u8 {
    let mut weapon_handle = read_u64(vp, addr + WEAPON);
    weapon_handle &= 0xffff;

    let weapon_entity = read_u64(vp, base + CL_ENTITYLIST + (weapon_handle << 5));
    let index = read_u16(vp, weapon_entity + WEAPON_NAME);
    let projectile_speed = read_f32(vp, weapon_entity + BULLET_SPEED);
    let projectile_scale = read_f32(vp, weapon_entity + BULLET_SCALE);
    let zoom_fov = read_f32(vp, weapon_entity + ZOOM_FOV);
    let ammo = read_u16(vp, weapon_entity + AMMO);
    let semi_auto = read_u16(vp, weapon_entity + SEMI_AUTO);
    let selected_slot = read_u8(vp, addr + SELECTED_SLOT);
    println!("wephandle -> {:x}", weapon_handle);
    println!("wep_entity -> {:x}", weapon_entity);
    println!("index -> {:?}", index);
    println!("projectile_speed -> {:?}", projectile_speed);
    println!("projectile_scale -> {:?}", projectile_scale);
    println!("zoom_fov -> {:?}", zoom_fov);
    println!("ammo -> {:?}", ammo);
    println!("semi_auto -> {:?}", semi_auto);
    selected_slot
}






