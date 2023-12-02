use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::thread::sleep;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;
use crate::config::Config;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MemChunk {
    pub cmd: CMD,
    pub addr: u64,
    pub size: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum CMD {
    #[default]
    Read,
    Write,
    GetBase
}

pub async fn main_network(query_sender: Sender<Vec<MemChunk>>, results_receiver: Receiver<Vec<MemChunk>>, cfg: Config) {
    let local_addr = format!("{}:{}", cfg.local_ip, cfg.local_port);
    let mut send_buf: [u8; 65535] = [0; 65535];
    let mut recv_buf: [u8; 65535] = [0; 65535];
    let local = UdpSocket::bind(local_addr).await.expect("bind ip failed");
    loop {
        // query recv -> send to interface
        if local.try_recv_from(&mut recv_buf).is_ok() {
            // decode
            let recv: String = match bincode::serde::decode_from_slice(&recv_buf, bincode::config::standard()) {
                Ok((recv, _usize)) => {
                    // println!("{:?}", recv);
                    recv
                }
                Err(e) => {
                    println!("decode string -> {:?}", e);
                    continue;
                }
            };
            let chunks: Vec<MemChunk> = match serde_json::from_str(recv.as_str()) {
                Ok(chunks) => { chunks }
                Err(e) => {
                    println!("decode config json -> {:?}", e);
                    continue;
                }
            };
            // send
            if query_sender.try_send(chunks).is_ok() {}
        };


        // interface results recv -> send to remote address
        if let Ok(chunks) = results_receiver.try_recv() {
            // encode
            let chunks_json = match serde_json::to_string(&chunks) {
                Ok(v) => {
                    // println!("{:?}", v);
                    v
                }
                Err(e) => {
                    println!("encode data json -> {:?}", e);
                    continue;
                }
            };
            let chunks_length = match bincode::serde::encode_into_slice(chunks_json, &mut send_buf, bincode::config::standard()) {
                Ok(v) => {
                    /*println!("length -> {:?}", v);*/
                    v
                }
                Err(e) => {
                    println!("encode string -> {:?}", e);
                    continue;
                }
            };
            // send
            if local.try_send_to(&send_buf[..chunks_length], SocketAddr::new(IpAddr::V4(cfg.remote_ip), cfg.remote_port)).is_ok() { /*println!("{:?}", data)*/ };
        }
        sleep(Duration::from_millis(100));
    }
}