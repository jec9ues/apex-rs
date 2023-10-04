use std::collections::HashMap;
use std::time::Instant;
use egui_backend::egui::Pos2;


use rdev::Key;
use crate::data::{Character, InputSystem};
#[derive(Debug, Default, Copy, Clone)]
pub struct Config {
    // pub character: HashMap<Character, CharacterConfig>,
    pub screen: ScreenConfig,
    pub aim: AimConfig,
    pub glow: GlowConfig,
    pub esp: EspConfig,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct CharacterConfig {
    pub hitbox: HitboxConfig,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct HitboxConfig {

}
#[derive(Debug, Default, Copy, Clone)]
pub struct AimConfig {
    pub distance: f32,
    pub aim_assist: AimAssistConfig,
    pub trigger_bot: TriggerBotConfig,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct AimAssistConfig {
    pub enable: bool,
    pub yaw_smooth: f32,
    pub pitch_smooth: f32,
    pub key: InputSystem,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct TriggerBotConfig {
    pub enable: bool,
    pub key: InputSystem,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct EspConfig {
    pub enable: bool,
    pub distance: f32,
    pub delay: u16,

}

#[derive(Debug, Default, Copy, Clone)]
pub struct GlowConfig {
    pub player_glow: PlayerGlowConfig,
    pub item_glow: ItemGlowConfig,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerGlowConfig {
    pub enable: bool,
    pub delay: u16,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ItemGlowConfig {
    pub enable: bool,
    pub delay: u16,
}
#[derive(Debug, Default, Copy, Clone)]
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
}

#[derive(Debug, Default, Copy, Clone)]
pub struct MenuConfig {
    pub fps: f32,
    pub config: Config,
}

impl MenuConfig {
    #[inline]
    pub fn update_fps(&mut self, last_frame_time: Instant) {
        self.fps = 1.0 / (Instant::now() - last_frame_time).as_secs_f32();
    }
}


