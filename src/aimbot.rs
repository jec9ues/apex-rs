use std::sync::mpsc::Receiver;
use crate::mem::*;
use crate::cache::*;
use crate::constants::offsets::*;
use crate::data::*;
use crate::function::*;
use crate::math::*;
use mouse_rs::*;
pub fn main_aimbot(data: Receiver<Data>) {
    let mouse = Mouse::new();
    mouse.move_to()
}