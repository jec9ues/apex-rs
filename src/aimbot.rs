use crossbeam_channel::{Receiver, TryRecvError};
use memprocfs::VmmProcess;
use mouse_rs::Mouse;
use crate::mem::*;
use crate::cache::*;
use crate::constants::offsets::*;
use crate::data::*;
use crate::function::*;
use crate::math::*;
use rdev::{listen, Event, EventType, Key, Button, simulate};
use crate::main;
//TODO: memory aimbot; kmbox net lib
pub fn main_aimbot(recv: Receiver<VmmProcess>) {
    match recv.try_recv() {
        Ok(vp) => {
            println!("pid -> {}", vp.pid)
        }
        Err(_) => {}
    }
/*    fn send(event_type: &EventType) {
        match simulate(event_type) {
            Ok(()) => (),
            Err(SimulateError) => {
                println!("We could not send {:?}", event_type);
            }
        }
    }

    let callback = move |event: Event| {
        // println!("My callback {:?}", event);

        let aim = Key::KeyP;
        match event.event_type {
            EventType::KeyPress(key) => {
                if key == aim {};
                println!("User wrote")
            },
            EventType::ButtonPress(Button::Left) => {

            },
            _ => {}
        }
    };


    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }*/


}