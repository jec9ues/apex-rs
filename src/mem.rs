use std::{env, ptr};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossbeam_channel::{Receiver, Sender};
use mem_struct::data_struct::apex::constants::offsets::{CL_ENTITYLIST, LEVEL_NAME, MODEL_NAME, SIGN_NAME};
use mem_struct::data_struct::apex::model_name::{ ModelNamePlayer};
use mem_struct::data_struct::apex::structs::EntityList;
use memprocfs::*;
use pretty_hex::PrettyHex;
use serde::{Deserialize, Serialize};
use crate::interface::{Interface, Readable};
use crate::network::{CMD, MemChunk};


/// send: data, recv: config
pub fn main_mem(results_sender: Sender<Vec<MemChunk>>, query_receiver: Receiver<Vec<MemChunk>>) {
    let mut path = PathBuf::new();
    match env::current_dir() {
        Ok(mut current_dir) => {
            if cfg!(windows) {
                current_dir.push("vmm.dll");
            } else {
                current_dir.push("vmm.so");
            }
            println!("{:?} -> {:?}", env::consts::OS, current_dir);
            path = current_dir
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
    println!("DMA device initializing");
    let path = path.to_str().unwrap();
    let vmm_args = ["-device", "fpga", "-memmap", "auto"].to_vec();

    let vmm_args = ["-printf", "-v", "-waitinitialize", "-device", "qemu://shm=qemu-win10.mem,qmp=/tmp/qmp-win10.sock", "-vm"].to_vec();
    let vmm = Vmm::new("/root/vmm.so", &vmm_args).unwrap();
    println!("vmm result = ok!");

    println!("========================================");
    println!("Vmm.set_config():");
    let _ = vmm.set_config(CONFIG_OPT_REFRESH_ALL, 1);
    println!("caches full refresh: -> Ok");

    println!("========================================");
    println!("vmm.process_from_name():");
    let vp = vmm.process_from_name("r5apex.exe").unwrap();
    println!("r5apex.exe Pid -> {}", vp.pid);

    println!("========================================");
    println!("vmmprocess.get_module_base():");
    let base = if let Ok(base) = vp.get_module_base("r5apex.exe") {
        println!("r5apex.exe base -> {:x}", base);
        base
    } else {
        panic!("r5apex.exe base address not found!");
    };
    //
    // let mut mem_chunks: Vec<MemChunk> = Vec::new();

    loop {
        let entitylist = vp.read_op::<EntityList>(base + CL_ENTITYLIST, (60 << 5)).unwrap();

        for (index, ent) in entitylist.data {
            ent.map(|v| {
                let model_ptr = vp.read::<u64>(v + 0x0030).unwrap();
                let h = vp.read_direct(model_ptr, 0x60).unwrap();
                vp.read_op::<ModelNamePlayer>(model_ptr, 0x60);
                // println!("model -> {:?}", h.hex_dump());

                // let h = vp.read_direct(v + MODEL_NAME, 0x10).unwrap();
                // println!("model -> {:?}", h.hex_dump());
            });
        }
        // println!("{:?}", vp.read_op::<EntityList>(base + CL_ENTITYLIST, (60 << 5)));
        sleep(Duration::from_secs(5))
        // if let Ok(v) = query_receiver.try_recv() {
        //     if v.is_empty() { continue };
        //     for mut i in v {
        //         match i.cmd {
        //             CMD::Read => { i.data = read_mem(&vp, i.addr, i.size as usize); }
        //             CMD::Write => { write_mem(&vp, i.addr, i.data.to_vec()); continue }
        //             CMD::GetBase => { i.addr = base;  }
        //
        //         }
        //         mem_chunks.push(i);
        //
        //     }
        // };
        // let start = Instant::now();
        // for _i in 1..10000 {
        //     read_mem(&vp, base + 0x16966f0, 65000 ).hex_dump();
        // };
        // let end = Instant::now() - start;
        // println!("loop 10000 -> {:?}", end);
        //
        // if results_sender.try_send(mem_chunks.clone()).is_ok() { mem_chunks.clear() };
    }

}

