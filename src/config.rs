use std::collections::HashMap;
use egui_backend::egui::Pos2;


use rdev::Key;
use crate::data::{Character, InputSystem};
#[derive(Debug, Default, Clone)]
pub struct Config {
    pub character: HashMap<Character, CharacterConfig>,
    pub aim: AimConfig,
    pub screen: ScreenConfig,
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
    pub aim_assist: AimAssistConfig,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct AimAssistConfig {
    pub enable: bool,
    pub smooth: f32,
    pub key: InputSystem,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct TriggerBotConfig {
    pub enable: bool,
    pub key: InputSystem,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct EspConfig {
    pub distance: f32,

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
