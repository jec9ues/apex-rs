extern crate core;


use log4rs;
use std::mem::size_of;
use std::{env, thread, time};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossbeam_channel::*;
use egui_backend::egui::Pos2;
use memprocfs::*;
use pretty_hex::PrettyHex;
use crate::cache::*;
use crate::constants::offsets::*;
use crate::data::*;
use crate::function::*;
use crate::math::*;
use rdev::*;
use rdev::EventType::KeyPress;
use crate::config::{Config, ScreenConfig};
use crate::kmbox_bpro::main_kmbox_bpro;

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
    const SIZE: usize = 32; // 假设最大字符串长度为 32，可以根据实际情况调整

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

{
            // 如果没有找到零字节，使用整个数据
            match String::from_utf8(Vec::from(slice)) {
                Ok(s) => s,
                Err(_) => String::new(),
            }
        }
    }
}

/// send: data, recv: config
pub fn main_mem(data_sender: Sender<Data>, config_recv: Receiver<Config>, restart_receiver: Receiver<bool>) {
    let mut path = PathBuf::new();
    match env::current_dir() {
        Ok(mut current_dir) => {
            current_dir.push("vmm.dll");
            println!("Current directory: {:?}", current_dir);
            path = current_dir
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
    println!("DMA for Apex - START");
    let path = path.to_str().unwrap();
    println!("{:?}", path);
    let vmm_args = ["-device", "fpga", "-memmap", "auto"].to_vec();
    let vmm = Vmm::new(path, &vmm_args).unwrap();
    // let vmm = Vmm::new("D://ovo-rs//vmm.dll", &vmm_args).unwrap();
    println!("vmm result = ok!");

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

    loop {
        if read_string(vp, base + LEVEL_NAME) == "mp_lobby" {
            println!("in lobby, take a short break");
            sleep(Duration::from_secs(20));
        } else {
            break
        }
    }


    // entity.update_pointer(vp);
    // entity.update_bone_index(vp);
    let mut data = Data::default();
    data.initialize(vp, base);
    let mut delay: u16 = 0;

    // data.config.load();
    loop {
        let vp = vp.clone();
        println!("{}", read_string(vp, base + LEVEL_NAME));
        if delay == u16::MAX {
            delay = u16::MIN;
            if read_string(vp, base + LEVEL_NAME) == "mp_lobby" {

                // mp_rr_olympus_mu1_night
                println!("in lobby, take a short break");
                break
            }
        }
        match config_recv.try_recv() {
            Ok(config) => {
                // println!("in");
                data.config = config;
                // data.config.screen = ScreenConfig::new([data.config.screen.size.x, data.config.screen.size.y]);
                // println!("config -> {:?}", data.config);
            }
            Err(_) => {}
        }

        match restart_receiver.try_recv() {
            Ok(restart) => {
                if restart { break }
            }
            Err(_) => {}
        }

        // entity.status.update(vp, entity.pointer, base);
        // 12 blue  13 22 normal 25 yellow 45 47 51 flash no outline 174 small outline


        // println!("status -> {:?}", entity.status);
        let start_time = Instant::now();
        /*        for pos in data.cache_data.get_players_bones_position(vp) {

                    println!("pos1 -> {:?}", pos);
                    println!("pos -> {:?}",  world_to_screen(local_player.view_matrix, pos, Pos2::new(2560.0, 1440.0)))
                };*/
        data.update_status(vp);
        data.update_cache(vp);
        // let raw = read_mem(vp, data.base + NST_WEAPON_NAMES, 0x10);
        // println!("raw -> {:?}", raw.hex_dump());
        // let mut wp = WeaponX::default();
        // wp.update(vp, data.cache_data.local_player.pointer, data.base);

        // for (_, player) in &data.cache_data.players {
        //
        //     let cacu = skynade_angle(wp, &data.cache_data.local_player, &player.position);
        //     let cacu = skynade_angle(wp, &data.cache_data.local_player, &player.position);
        //     println!("pos -> {:?}", player.position);
        //     println!("skynade -> {:?}", (cacu.0.to_degrees(), cacu.1.to_degrees()));
        //     data.grenade = [cacu.0.to_degrees(), cacu.1.to_degrees()];
        //     data.cache_data.local_player.set_angle(vp, -cacu.0.to_degrees(), cacu.1.to_degrees());
        // }

        println!("distance_2d -> {:?}", data.cache_data.target.distance_2d);
        println!("distance -> {:?}", data.cache_data.target.distance);
        // println!("position -> {:?}", data.cache_data.target.position);
        // println!("my position -> {:?}", data.cache_data.local_player.position);
        // println!("my pitch -> {:?}", data.cache_data.local_player.pitch);
        // data.cache_data.local_player.set_pitch(vp, data.config.aim.pitch);

        if data.config.esp.enable || data.config.aim.aim_assist.enable || data.config.aim.trigger_bot.enable {
            data.update_basic(vp, data.config.esp.distance); // ~ 5ms per player
            data.update_target(vp, data.config.aim.distance, data.config.screen.size);

            if delay & data.config.esp.delay == 0 {
                data.update_basic(vp, f32::MAX); // ~ 5ms per player
            }
        }
        if data.config.glow.player_glow.enable && (delay & data.config.glow.player_glow.delay == 0) {
/*            let mut i = 0.0; // 当前位置

            loop {
                let color: [f32; 3] = rainbow_color(i);

                // 输出 RGB 值（范围在 0.0 到 1.0 之间）
                // println!("RGB({}, {}, {})", r, g, b);

                i += 0.01; // 调整步长

                if i >= 1.0 {
                    i = 0.0; // 重置到0，实现循环
                }
            }
            fn rainbow_color(t: f32) -> [f32; 3] {
                // 根据标准化值 t 计算 RGB 颜色
                let r = (t * 2.0 - 1.0).max(0.0).min(1.0); // 红色分量
                let g = (-t * 2.0 + 2.0).max(0.0).min(1.0); // 绿色分量
                let b = (t * 2.0).max(0.0).min(1.0); // 蓝色分量

                [r, g, b]
            }*/
            if data.cache_data.target.status.visible() {
                im_player_glow(vp, data.base, 60, data.config.glow.visible_color, false);
            } else {
                im_player_glow(vp, data.base, 60, data.config.glow.invisible_color, false);
            }

        }

        if data.config.glow.item_glow.enable && (delay & data.config.glow.item_glow.delay == 0) {
            im_player_glow(vp, data.base, 15000,data.config.glow.visible_color,  true);
        }
        // println!("target name -> {}", data.cache_data.target.status.name);
        // im_player_glow(vp, base, data.cache_data.local_player.status.team);
        // data.re_cache_pointer(vp);
        // println!("{:?}", data.cache_data.target);
        if data.config.aim.aim_assist.enable || data.config.aim.trigger_bot.enable {
            let target_bone = data.cache_data.target.get_nearest_bone(data.config.screen.center());
            let pitch = calculate_desired_pitch(data.cache_data.local_player.camera_position, target_bone.position);
            let yaw = calculate_desired_yaw(data.cache_data.local_player.camera_position, target_bone.position);
            // let angle_delta = calculate_angle_delta(data.cache_data.local_player.yaw, yaw);
            // let pitch_delta = calculate_pitch_angle_delta(data.cache_data.local_player.pitch, pitch);

            // let angle_delta = calculate_angle_delta(data.cache_data.local_player.yaw, yaw);
            // let pitch_delta = calculate_pitch_angle_delta(data.cache_data.local_player.pitch, pitch);

            let distance_to_target = data.cache_data.target.position_2d.distance(data.config.screen.center());

            let angle_delta = calculate_angle_delta(data.cache_data.local_player.yaw, yaw);
            let angle_delta_smooth = calculate_delta_smooth(distance_to_target, data.config.aim.aim_assist.yaw_smooth, data.config.aim.aim_assist.yaw_curve_factor);

            let pitch_delta = calculate_pitch_angle_delta(data.cache_data.local_player.pitch, pitch);
            let pitch_delta_smooth = calculate_delta_smooth(distance_to_target,  data.config.aim.aim_assist.pitch_smooth, data.config.aim.aim_assist.pitch_curve_factor);

            let new_yaw = flip_yaw_if_needed(data.cache_data.local_player.yaw + angle_delta / angle_delta_smooth);
            let new_pitch = data.cache_data.local_player.pitch + pitch_delta / pitch_delta_smooth;
            // println!("calculate pitch -> {}, yaw -> {}", new_pitch, new_yaw);
            if data.cache_data.target.status.visible() {
                if data.key.get_key_state(data.config.aim.aim_assist.key) || data.key.get_key_state(data.config.aim.aim_assist.key2) {
                    // data.cache_data.local_player.set_yaw(vp, new_yaw);
                    // data.cache_data.local_player.set_pitch(vp, new_pitch);
                    if data.config.aim.aim_assist.kmbox {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        // 在 Tokio 运行时中执行异步函数
                        rt.block_on(main_kmbox_bpro(pitch_delta / pitch_delta_smooth, angle_delta / angle_delta_smooth, data.config.aim.aim_assist.pitch_rate, data.config.aim.aim_assist.yaw_rate)).expect("TODO: panic message");

                    } else {
                        data.cache_data.local_player.set_angle(vp, new_pitch, new_yaw); // 500 µs
                    }

                }
                if data.config.aim.trigger_bot.enable {
                    if data.cache_data.target.status.target() {
                        if data.key.get_key_state(data.config.aim.trigger_bot.key) {
                            // data.cache_data.local_player.set_yaw(vp, new_yaw);
                            // data.cache_data.local_player.set_pitch(vp, new_pitch);
                            if data.cache_data.target.hitbox_check(data.config.screen.center(), data.config.aim.trigger_bot.hitbox_size) {
                                sleep(Duration::from_micros(data.config.aim.trigger_bot.delay));
                                send(&EventType::ButtonPress(Button::Left));
                                send(&EventType::ButtonRelease(Button::Left));
                            }
                        }
                    }
                }
            }
        }

/*        data.cache_data.local_player.set_pitch(vp, pitch);
        let angle_delta_abs = angle_delta.abs();
        println!("slot -> {}", weaponx_entity(vp, data.cache_data.local_player.pointer, base));
        last_time = data.cache_data.target.status.last_crosshair_target_time;
        println!("calculate pitch -> {}, yaw -> {}", angle_delta, data.config.aim.aim_assist.yaw_smooth);
        println!("visible -> {}", data.cache_data.target.status.visible());
        last_vis = data.cache_data.target.status.last_visible_time;
        println!("button state -> {}", get_button_state(107, vp, base));
        println!("pitch -> {}, yaw -> {}", data.cache_data.local_player.pitch, data.cache_data.local_player.yaw);
        println!("calculate pitch -> {}, yaw -> {}", pitch, yaw);*/


        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time);
        // println!("Loop time -> {:?}", elapsed_time);

        data_sender.send(data.clone()).expect("data send failed");

        sleep(Duration::from_micros(100));
        // sleep(Duration::from_millis(1000));
        delay += 1
    }
}

