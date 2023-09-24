use crossbeam_channel::Receiver;
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
pub fn main_aimbot(recv_data: Receiver<Data>) {
    let mut data: Data = Data::default();
    fn send(event_type: &EventType) {
        match simulate(event_type) {
            Ok(()) => (),
            Err(SimulateError) => {
                println!("We could not send {:?}", event_type);
            }
        }
    }

    let callback = move |event: Event| {
        // println!("My callback {:?}", event);
        match recv_data.try_recv() {
            Ok(da) => {
                // println!("Received message from thread {:?}", data);
                data = da;
            }
            Err(_) => {}
        };
        let aim = Key::KeyP;
        match event.event_type {
            EventType::KeyPress(key) => {
                if key == aim {};
                println!("User wrote")
            },
            EventType::ButtonPress(Button::Left) => {

                let target = data.get_near_player();
                println!("{:?} {:?}", target.hitbox.head.position_2d.x, target.hitbox.head.position_2d.y);
                send(&EventType::MouseMove { x: target.hitbox.head.position_2d.x as f64, y: target.hitbox.head.position_2d.y as f64 });

            },
            _ => {}
        }
    };


    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }


}