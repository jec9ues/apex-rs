use std::collections::HashMap;
use std::sync::Condvar;
use egui_backend::egui::*;
use egui_backend::egui::epaint::PathShape;
use egui_window_glfw_passthrough::glfw::WindowEvent::Pos;
use memprocfs::VmmProcess;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::data::*;
use crate::function::*;
use crate::math::*;
use crate::menu::dbg_player;

pub enum Cache {
    High, // near position
    Medium, // far position
    Low, // dead player
    Static, // pointer
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CachePtr {
    pub cache_high: Vec<u64>,
    pub cache_medium: Vec<u64>,
    pub cache_low: Vec<u64>,
    pub cache_static: Vec<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheData {
    pub local_player: LocalPlayer,
    pub target: Player,
    pub players: HashMap<u64, Player>,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub base: u64,
    pub cache_pointer: CachePtr,
    pub cache_data: CacheData,
    pub grenade: [f32; 2],
    pub key: KeyData,
    pub config: Config,
    // pub table: DataTable,
}

impl Data {
    //TODO: improve get player by distance, health, fov

    pub fn get_near_player(&self) -> Player {
        let mut near_player: &Player = &Default::default();
        let mut last_distance: f32 = 0.0;
        for pointer in &self.cache_pointer.cache_high {
            if let Some(player) = self.cache_data.players.get(&pointer) {
                if player.distance == 0.0 { continue };
                println!("distance -> {}", player.distance);
                if last_distance < player.distance && player.status.team != self.cache_data.local_player.status.team && player.status.dead == 0{
                    near_player = player;
                }
            }
        }
        near_player.clone()
    }
    pub fn get_near_crosshair_player(&self) -> Player {
        let mut near_player: &Player = &Default::default();
        let mut last_distance: f32 = 999.0;
        let mut last_dis: f32 = 999.0;
        for pointer in &self.cache_pointer.cache_high {
            if let Some(player) = self.cache_data.players.get(&pointer) {
                // println!("ptr -> {:?}", player.pointer);
                // println!("head pos -> {:?}", player.hitbox.head.position_2d);
                // println!("distance -> {}", player.distance);
                if player.position_2d == Pos2::ZERO || player.distance > 150.0 || player.status.dead > 0 || player.status.knocked > 0 { continue }
                // if player.distance > 150.0 { continue }
                let dis = distance2d(&self.config.screen.center, &player.position_2d);
                if last_distance > dis{
                    last_distance = dis;
                    last_dis = player.distance;
                    near_player = player;
                }
            }
        }
        // println!("distance -> {}", near_player.status.platform_id);
        near_player.clone()
    }

    pub fn get_near_crosshair_target(&self, distance: f32, team_check: bool) -> Player {
        let mut near_player: &Player = &Default::default();
        let mut last_distance: f32 = 999.0;
        for pointer in &self.cache_pointer.cache_medium {
            if let Some(player) = self.cache_data.players.get(&pointer) {

                if team_check {
                    if player.status.team == self.cache_data.local_player.status.team { continue }
                }

                if player.position_2d == Pos2::ZERO || player.distance > distance || player.status.dead > 0 || player.status.knocked > 0 { continue }

                let dis = distance2d(&self.config.screen.center(), &player.position_2d);
                if last_distance > dis{
                    last_distance = dis;
                    near_player = player;
                }
            }
        }
        // println!("distance -> {}", near_player.status.platform_id);
        near_player.clone()
    }

    pub fn dbg_view(&self, ui: &mut Ui) {
        for i in &self.cache_data.players {

            dbg_player(i.1, ui);
        }
    }

}