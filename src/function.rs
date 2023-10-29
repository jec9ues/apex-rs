use std::fmt::Error;
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
use rdev::{EventType, simulate};
use crate::data::{Bone, GRENADE_PITCHES, launch2view, LocalPlayer, Pitch, Player, Pos3, WeaponX};
use crate::egui_overlay::EguiOverlay;


fn move_and_press() {
    let mouse = Mouse::new();
    mouse.move_to(500, 500).expect("Unable to move mouse");
    mouse.press(&Keys::RIGHT).expect("Unable to press button");
    mouse.release(&Keys::RIGHT).expect("Unable to release button");
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


pub fn send(event_type: &EventType) {
    let delay = Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    sleep(delay);
}

pub struct FpsCounter {
    frame_count: u32,
    last_fps_update: Instant,
    fps: f32,
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {
            frame_count: 0,
            last_fps_update: Instant::now(),
            fps: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_fps_update);
        if elapsed >= Duration::from_secs(1) {
            // Calculate FPS
            self.fps = (self.frame_count as f32) / elapsed.as_secs_f32();
            // Reset counters
            self.frame_count = 0;
            self.last_fps_update = now;
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}




pub fn calculate_delta_smooth(distance: f32, smooth: f32, curve_factor: f32) -> f32 {
    let smooth_factor = 1.0 / distance; // distance reciprocal
    let angle_delta =  smooth / smooth_factor / curve_factor;
    // println!("distance -> {}, smooth_factor -> {} angle_delta -> {}", distance, smooth_factor, angle_delta);
    angle_delta
}


pub fn skynade_angle(weapon: WeaponX, local: &LocalPlayer, target: &Pos3) -> (f32, f32) {

    let (lob, pitches, z_offset): (bool, &[Pitch], f32) =(true, &GRENADE_PITCHES, 70.0);

    let g = 750.0 * 1.0;
    let v0 = 10000.0;

    let delta = target.sub(&local.position);
    let delta = delta.add(&delta.muls(20.0 / delta.len()));
    let dx = f32::sqrt(delta.x * delta.x + delta.y * delta.y);
    let dy = delta.y + z_offset;

    let calc_angle = if lob { lob_angle } else { optimal_angle };
    if let Some(launch_pitch) = calc_angle(dx, dy, v0, g) {
        let view_pitch = launch2view(pitches, launch_pitch);
        return (view_pitch, target.sub(&local.position).qangle().y.to_radians());
        // return (view_pitch, sdk::qangle(sdk::sub(*target, local.view_origin))[1].to_radians());
    }
    else {
        return Default::default();
    }

    fn optimal_angle(x: f32, y: f32, v0: f32, g: f32) -> Option<f32> {
        let root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0 * y * v0 * v0);
        if root < 0.0 {
            return None;
        }
        let root = f32::sqrt(root);
        let slope = (v0 * v0 - root) / (g * x);
        Some(f32::atan(slope))
    }
    fn lob_angle(x: f32, y: f32, v0: f32, g: f32) -> Option<f32> {
        let root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0 * y * v0 * v0);
        if root < 0.0 {
            return None;
        }
        let root = f32::sqrt(root);
        let slope = (v0 * v0 + root) / (g * x);
        Some(f32::atan(slope))
    }
}