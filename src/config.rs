use std::collections::HashMap;
use std::{fs, mem};
use std::time::Instant;
use egui_backend::egui::Pos2;


use rdev::Key;
use serde::{Deserialize, Serialize};
use crate::data::{Character, InputSystem};
use crate::function::FpsCounter;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct Config {
    // pub character: HashMap<Character, CharacterConfig>,
    pub screen: ScreenConfig,
    pub aim: AimConfig,
    pub glow: GlowConfig,
    pub esp: EspConfig,
}
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct CharacterConfig {
    pub hitbox: HitboxConfig,
}
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct HitboxConfig {

}
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct AimConfig {
    pub distance: f32,
    pub team_check: bool,
    pub aim_assist: AimAssistConfig,
    pub trigger_bot: TriggerBotConfig,
}
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct AimAssistConfig {
    pub enable: bool,
    pub yaw_curve_factor: f32,
    pub pitch_curve_factor: f32,
    pub yaw_smooth: f32,
    pub pitch_smooth: f32,
    pub key: u8,
    pub key2: u8,
}
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct TriggerBotConfig {
    pub enable: bool,
    pub delay: u64,
    pub hitbox_size: f32,
    pub key: u8,
}
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct EspConfig {
    pub enable: bool,
    pub distance: f32,
    pub delay: u16,

}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct GlowConfig {
    pub visible_color: [f32; 3],
    pub invisible_color: [f32; 3],
    pub player_glow: PlayerGlowConfig,
    pub item_glow: ItemGlowConfig,
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct PlayerGlowConfig {
    pub enable: bool,
    pub delay: u16,
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct ItemGlowConfig {
    pub enable: bool,
    pub delay: u16,
}
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct ScreenConfig {
    pub size: Pos2,
    pub center: Pos2,
}
impl ScreenConfig {
    pub fn new(value: [f32; 2]) -> Self {
        ScreenConfig {
            size: Pos2::from(value),
            center: Pos2::from([value[0] / 2.0, value[1] / 2.0]),
        }
    }
    pub fn center(&self) -> Pos2 {
        Pos2::new(self.size.x / 2.0, self.size.y / 2.0)
    }
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct MenuConfig {
    pub config: Config,
}


impl MenuConfig {
    pub const PATH: &'static str = "config.json";
    pub fn load(&mut self) {
        let config = fs::read_to_string(MenuConfig::PATH).expect("Unable to read file");
        let mut res: Config = serde_json::from_str(&config).expect("Unable to parse");
        println!("load -> {:?}", res);
        mem::swap(&mut self.config, &mut res);
    }
    pub fn save(&self) {
        let res: String = serde_json::to_string(&self.config).expect("Uable to convert config");
        println!("write -> {:?}", res);
        match fs::write(MenuConfig::PATH, res) {
            Ok(_) => { println!("success write file")}
            Err(err) => { eprintln!("Error: {:?}", err); }
        }
    }

}

