use std::collections::HashMap;
use rdev::Key;
use crate::data::Character;

pub struct Config {
    pub character: HashMap<Character, CharacterConfig>,
    pub aim: AimConfig,

}
//hitbox size, ..
pub struct CharacterConfig {
    pub hitbox: HitboxConfig,
}

pub struct HitboxConfig {

}

pub struct AimConfig {
    pub aim_assist: AimAssistConfig,
}

pub struct AimAssistConfig {
    pub enable: bool,
    pub smooth: f32,
    pub key: Key,
}

pub struct TriggerBotConfig {
    pub enable: bool,
    pub key: Key,
}

pub struct EspConfig {
    pub distance: f32,

}
