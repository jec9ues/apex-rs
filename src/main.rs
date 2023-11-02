pub mod mem;
pub mod constants;
pub mod function;
pub mod math;
pub mod data;
pub mod cache;
pub mod config;
pub mod network;


use std::fmt::{Debug, Display};
use std::ops::RangeInclusive;
use std::sync::Once;
use std::thread;

use crossbeam_channel::*;
use log4rs;
use crate::cache::Data;
use crate::mem::*;
use rand::Rng;
use crate::config::{Config, MenuConfig};
use crate::network::main_network;


fn main() {
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let (config_sender, config_receiver) = bounded::<Config>(1);

    let (data_sender, data_receiver) = bounded::<Data>(1);

    let (restart_sender, restart_receiver) = bounded::<bool>(1);

    let mem = thread::spawn(move || {
        loop {
            main_mem(data_sender.clone(),
                     config_receiver.clone(),
                     restart_receiver.clone()
            );
        }
    });
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        main_network(
            data_receiver.clone(),
            config_sender.clone(),
            restart_sender.clone()
        )
    );

    mem.join().unwrap();
}