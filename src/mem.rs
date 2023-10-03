extern crate core;


use log4rs;
use std::mem::size_of;
use std::{thread, time};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossbeam_channel::*;
use egui_backend::egui::Pos2;
use memprocfs::*;
use crate::cache::*;
use crate::constants::offsets::*;
use crate::data::*;
use crate::function::*;
use crate::math::*;
use rdev::*;
use crate::config::ScreenConfig;

pub fn read_mem(vp: VmmProcess, addr: u64, size: usize) -> Vec<u8> {
    match vp.mem_read_ex(addr, size, FLAG_NOCACHE | FLAG_ZEROPAD_ON_FAIL | FLAG_NOPAGING) {
        Err(e) => {
            println!("vmmprocess.mem_read_ex(): fail [{}]", e);
            Vec::new() // 在错误情况下返回一个空的 Vec<u8>
        },
        Ok(data) => {
            data
        },
    }
}



pub fn read_u8(vp: VmmProcess, addr: u64) -> u8 {
    const SIZE: usize = size_of::<u8>();

    let data_read = read_mem(vp, addr, SIZE);

    // let res = u64::from_le_bytes(data_read.as_slice().to_owned().try_into().expect("Vec has unexpected length"));
    let res = u8::from_le_bytes(data_read.as_slice().to_owned().try_into().expect("Vec has unexpected length"));
    // println!("data: {:?} res: {}", data_read, res);
    res
}
pub fn read_u16(vp: VmmProcess, addr: u64) -> u16 {
    const SIZE: usize = size_of::<u16>();

    let data_read = read_mem(vp, addr, SIZE);

    let res = u16::from_le_bytes(data_read.as_slice().to_owned().try_into().expect("Vec has unexpected length"));
    // println!("data: {:?} res: {}", data_read, res);
    res
}
pub fn read_u32(vp: VmmProcess, addr: u64) -> u32 {
    const SIZE: usize = size_of::<u32>();

    let data_read = read_mem(vp, addr, SIZE);

    let res = u32::from_le_bytes(data_read.as_slice().to_owned().try_into().expect("Vec has unexpected length"));
    // println!("data: {:?} res: {}", data_read, res);
    res
}

pub fn read_i32(vp: VmmProcess, addr: u64) -> i32 {
    const SIZE: usize = size_of::<i32>();

    let data_read = read_mem(vp, addr, SIZE);

    let res = i32::from_le_bytes(data_read.as_slice().to_owned().try_into().expect("Vec has unexpected length"));
    // println!("data: {:?} res: {}", data_read, res);
    res
}
pub fn read_u64(vp: VmmProcess, addr: u64) -> u64 {
    const SIZE: usize = size_of::<u64>();

    let data_read = read_mem(vp, addr, SIZE);

    let res = u64::from_le_bytes(data_read.as_slice().to_owned().try_into().expect("Vec has unexpected length"));
    // println!("data: {:?} res: {}", data_read, res);
    res
}
pub fn read_f32(vp: VmmProcess, addr: u64) -> f32 {
    const SIZE: usize = size_of::<f32>();

    let data_read = read_mem(vp, addr, SIZE);


    let mut bytes: [u8; SIZE] = [0; SIZE];
    bytes.copy_from_slice(&data_read);

    let res = f32::from_le_bytes(bytes);

    res
}



pub fn read_f32_vec(vp: VmmProcess, addr: u64, amount: usize) -> Vec<f32> {
    const SIZE: usize = size_of::<f32>();

    let data_read = read_mem(vp, addr, amount * SIZE);

    let mut f32_values: Vec<f32> = Vec::with_capacity(amount);

    for chunk in data_read.chunks_exact(SIZE) {
        let mut array: [u8; SIZE] = [0; SIZE];
        array.copy_from_slice(chunk);
        let f32_value = f32::from_le_bytes(array);
        f32_values.push(f32_value);
    }

    f32_values
}

pub fn read_string(vp: VmmProcess, addr: u64) -> String {
    const SIZE: usize = 24; // 假设最大字符串长度为 32，可以根据实际情况调整

    let data = read_mem(vp, addr, SIZE);

    if let Some(zero_index) = data.iter().position(|&byte| byte == 0) {
        // 如果找到第一个零字节，只取到第一个零字节之前的部分
        let truncated_data = &data[..zero_index];
        match String::from_utf8(truncated_data.to_vec()) {
            Ok(s) => s, // 如果成功转换为 String，返回 String 值
            Err(_) => String::new(), // 如果无法转换为有效的 UTF-8，返回空字符串或者其他错误处理方式
        }
    } else {
        // 如果没有找到零字节，使用整个数据
        match String::from_utf8(data) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }
}


pub fn write_mem(vp: VmmProcess, addr:u64, data: Vec<u8>) {
    match vp.mem_write(addr, &data) {
        Err(e) => { println!("vmmprocess.mem_write(): fail [{}]", e) },
        Ok(_) => {  }
    }
}
pub fn write_u8(vp: VmmProcess, addr:u64, value: u8) {
    write_mem(vp, addr, value.to_le_bytes().to_vec())
}
pub fn write_u16(vp: VmmProcess, addr:u64, value: u16) {
    write_mem(vp, addr, value.to_le_bytes().to_vec())
}
pub fn write_u32(vp: VmmProcess, addr:u64, value: u32) {
    write_mem(vp, addr, value.to_le_bytes().to_vec())
}
pub fn write_i32(vp: VmmProcess, addr:u64, value: i32) {
    write_mem(vp, addr, value.to_le_bytes().to_vec())
}
pub fn write_u64(vp: VmmProcess, addr:u64, value: u64) {
    write_mem(vp, addr, value.to_le_bytes().to_vec())
}
pub fn write_f32(vp: VmmProcess, addr:u64, value: f32) {
    write_mem(vp, addr, value.to_le_bytes().to_vec())
}

pub fn write_f32_vec(vp: VmmProcess, addr:u64, value: Vec<f32>) {
    write_mem(vp, addr, value
        .iter()
        .flat_map(|&f| f.to_le_bytes().to_vec())
        .collect())
}


pub struct ContinuingData {
    pub value: Vec<u8>,
}
impl ContinuingData {
    pub fn new(value: Vec<u8>) -> Self {
        ContinuingData { value }
    }

    pub fn read_u8(&self,  addr: usize) -> u8 {
        const SIZE: usize = size_of::<u8>();

        let slice = &self.value[addr..(addr + SIZE)];
        let bytes: [u8; SIZE] = slice.try_into().unwrap();

        let res = u8::from_le_bytes(bytes);
        // println!("data: {:?} res: {}", data_read, res);
        res
    }
    pub fn read_u16(&self, addr: u64) -> u16 {
        const SIZE: usize = size_of::<u16>();

        let slice = &self.value[addr as usize..(addr as usize + SIZE)];
        let bytes: [u8; SIZE] = slice.try_into().unwrap();

        let res = u16::from_le_bytes(bytes);
        // println!("data: {:?} res: {}", data_read, res);
        res
    }
    pub fn read_u32(&self, addr: u64) -> u32 {
        const SIZE: usize = size_of::<u32>();

        let slice = &self.value[addr as usize..(addr as usize + SIZE)];
        let bytes: [u8; SIZE] = slice.try_into().unwrap();

        let res = u32::from_le_bytes(bytes);
        // println!("data: {:?} res: {}", data_read, res);
        res
    }

    pub fn read_i32(&self, addr: u64) -> i32 {
        const SIZE: usize = size_of::<i32>();

        let slice = &self.value[addr as usize..(addr as usize + SIZE)];
        let bytes: [u8; SIZE] = slice.try_into().unwrap();

        let res = i32::from_le_bytes(bytes);
        // println!("data: {:?} res: {}", data_read, res);
        res
    }
    pub fn read_u64(&self, addr: u64) -> u64 {
        const SIZE: usize = size_of::<u64>();

        let slice = &self.value[addr as usize..(addr as usize + SIZE)];
        let bytes: [u8; SIZE] = slice.try_into().unwrap();

        let res = u64::from_le_bytes(bytes);
        // println!("data: {:?} res: {}", data_read, res);
        res
    }
    pub fn read_f32(&self, addr: u64) -> f32 {
        const SIZE: usize = size_of::<f32>();

        let slice = &self.value[addr as usize..(addr as usize + SIZE)];
        let bytes: [u8; SIZE] = slice.try_into().unwrap();

        let res = f32::from_le_bytes(bytes);

        res
    }



    pub fn read_f32_vec(&self, addr: u64, amount: usize) -> Vec<f32> {
        const SIZE: usize = size_of::<f32>();

        let slice = &self.value[addr as usize..amount * (addr as usize + SIZE)];

        let mut f32_values: Vec<f32> = Vec::with_capacity(amount);

        for chunk in slice.chunks_exact(SIZE) {
            let mut array: [u8; SIZE] = [0; SIZE];
            array.copy_from_slice(chunk);
            let f32_value = f32::from_le_bytes(array);
            f32_values.push(f32_value);
        }

        f32_values
    }

    pub fn read_string(&self, addr: u64) -> String {
        const SIZE: usize = 32; // 假设最大字符串长度为 32，可以根据实际情况调整

        let slice = &self.value[addr as usize..(addr as usize + SIZE)];
        let bytes: [u8; SIZE] = slice.try_into().unwrap();

        if let Some(zero_index) = bytes.iter().position(|&byte| byte == 0) {
            // 如果找到第一个零字节，只取到第一个零字节之前的部分
            let truncated_data = &bytes[..zero_index];
            match String::from_utf8(truncated_data.to_vec()) {
                Ok(s) => s, // 如果成功转换为 String，返回 String 值
                Err(_) => String::new(), // 如果无法转换为有效的 UTF-8，返回空字符串或者其他错误处理方式
            }
        } else {
            // 如果没有找到零字节，使用整个数据
            match String::from_utf8(Vec::from(slice)) {
                Ok(s) => s,
                Err(_) => String::new(),
            }
        }
    }
}


pub fn main_mem(sender: Sender<Vec<Pos2>>, data_sender: Sender<Data>, aimbot_send_data: Sender<Data>) {
    println!("DMA for Apex - START");


    let vmm_args = ["-device", "fpga", "-memmap", "auto"].to_vec();
    let vmm = Vmm::new("D:\\MEM\\vmm.dll", &vmm_args).unwrap();
    println!("vmm result = ok!");


    // For a full refresh of internal data caches.
    println!("========================================");
    println!("Vmm.set_config():");
    let _r = vmm.set_config(CONFIG_OPT_REFRESH_ALL, 1);
    println!("caches full refresh: -> Ok");


    println!("========================================");
    println!("vmm.process_from_name():");
    let vp = vmm.process_from_name("r5apex.exe").unwrap();
    println!("r5apex.exe Pid -> {}", vp.pid);


    let base:u64;
    println!("========================================");
    println!("vmmprocess.get_module_base():");
    if let Ok(ba) = vp.get_module_base("r5apex.exe") {
        println!("r5apex.exe base -> {:x}", ba);
        base = ba;
    } else {
        panic!("r5apex.exe base address not found!");

    }


    // entity.update_pointer(vp);
    // entity.update_bone_index(vp);
    let mut data = Data::default();
    data.initialize(vp, base);
    data.cache_data.local_player.update_pointer(vp, base);
    let mut tick = 20;

    data.config.screen = ScreenConfig::new([2560.0, 1440.0]);
    loop {
        // entity.status.update(vp, entity.pointer, base);
        // 12 blue  13 22 normal 25 yellow 45 47 51 flash no outline 174 small outline


        // println!("status -> {:?}", entity.status);
        let start_time = Instant::now();
        /*        for pos in data.cache_data.get_players_bones_position(vp) {

                    println!("pos1 -> {:?}", pos);
                    println!("pos -> {:?}",  world_to_screen(local_player.view_matrix, pos, Pos2::new(2560.0, 1440.0)))
                };*/
        data.update_basic(vp, 150.0); // ~ 5ms per player
        data.update_target(vp, 150.0);
        // println!("target name -> {}", data.cache_data.target.status.name);
        data.update_status(vp);
        if tick % 20 == 0 {

            data.update_basic(vp, 999.0); // ~ 5ms per player
            tick = 0;
        }

        // im_player_glow(vp, base, data.cache_data.local_player.status.team);

        // data.re_cache_pointer(vp);


        let pitch = calculate_desired_pitch(data.cache_data.local_player.hitbox.head.position, data.cache_data.target.hitbox.upper_chest.position);
        let yaw = calculate_desired_yaw(data.cache_data.local_player.hitbox.head.position, data.cache_data.target.hitbox.upper_chest.position);
        // data.cache_data.local_player.set_pitch(vp, pitch);
        let angle_delta = calculate_angle_delta(data.cache_data.local_player.yaw, yaw);
        let pitch_delta = calculate_pitch_angle_delta(data.cache_data.local_player.pitch, pitch);
        let angle_delta_abs = angle_delta.abs();

        let new_yaw = flip_yaw_if_needed(data.cache_data.local_player.yaw + angle_delta / 20.0);
        let new_pitch = data.cache_data.local_player.pitch + pitch_delta / 15.0;
        fn send(event_type: &EventType) {
            let delay = time::Duration::from_millis(20);
            match simulate(event_type) {
                Ok(()) => (),
                Err(SimulateError) => {
                    println!("We could not send {:?}", event_type);
                }
            }
            // Let ths OS catchup (at least MacOS)
            thread::sleep(delay);
        }
/*        for i in 109..200 {
            if get_button_state(i, vp, base) == 1 { println!("{}", i)}
        }*/
        // println!("slot -> {}", weaponx_entity(vp, data.cache_data.local_player.pointer, base));

        // last_time = data.cache_data.target.status.last_crosshair_target_time;

        if data.cache_data.target.status.visible(){
            if data.key.get_key_state(InputSystem::KEY_XBUTTON_LEFT_SHOULDER) || data.key.get_key_state(InputSystem::MOUSE_RIGHT){
                // data.cache_data.local_player.set_yaw(vp, new_yaw);
                // data.cache_data.local_player.set_pitch(vp, new_pitch);
                data.cache_data.local_player.set_angle(vp, new_pitch, new_yaw); // 500 µs

            }
        }

        // last_vis = data.cache_data.target.status.last_visible_time;
        // println!("button state -> {}", get_button_state(107, vp, base));
        // println!("pitch -> {}, yaw -> {}", data.cache_data.local_player.pitch, data.cache_data.local_player.yaw);
        // println!("calculate pitch -> {}, yaw -> {}", pitch, yaw);


        // let vh2s: Vec<Pos2> = data.cache_data.get_players_bones_position(vp)
        //     .iter()
        //     .map(|pos| world_to_screen(data.cache_data.local_player.view_matrix, *pos, Pos2::new(2560.0, 1440.0)))
        //     .collect(); // ~ 1.2 ms
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time);
        println!("Loop time -> {:?}", elapsed_time);
        // println!("matrix -> {:?}", data.cache_data.local_player.view_matrix);
        // println!("high -> {:?}", data.cache_pointer.cache_high);
        // println!("medium -> {:?}", data.cache_pointer.cache_medium);
        // println!("low -> {:?}", data.cache_pointer.cache_low);
        // sender.send(vh2s.clone()).expect("TODO: panic message");
        data_sender.send(data.clone()).expect("TODO: panic message");
        aimbot_send_data.send(data.clone()).expect("TODO: panic message");
        // println!("pos -> {:?}", data.cache_data.local_player.position);
        sleep(Duration::from_micros(10));
        tick += 1
    }
}

