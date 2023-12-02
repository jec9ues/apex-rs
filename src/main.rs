use std::thread;

use crossbeam_channel::bounded;
use crate::config::{init_cfg, read_config_from_file};
use crate::mem::main_mem;

use crate::network::{main_network, MemChunk};

pub mod mem;
pub mod network;
pub mod config;

pub fn main() {
    let cfg = init_cfg().unwrap_or(read_config_from_file("cfg.json").unwrap());
    let (results_sender, results_receiver) = bounded::<Vec<MemChunk>>(1);
    let (query_sender, query_receiver) = bounded::<Vec<MemChunk>>(1);

    let mem = thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(
            main_network(
                query_sender.clone(),
                results_receiver.clone(),
                cfg
            )
        );
    });

    let mem = thread::spawn(move || {
        main_mem(
            results_sender,
            query_receiver
        );
    });

/*    loop {
        let mut res: Vec<MemChunk> = Vec::default();
        for i in 0..4 {
            res.push(MemChunk::default());
        }
        let _ = results_sender.try_send(res).is_ok();
        let _ = query_receiver.try_recv().is_ok();
        // println!("pre test")
    }*/
    mem.join().unwrap();
    // main_example().unwrap();
}