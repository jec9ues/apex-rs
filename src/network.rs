use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread::sleep;
use std::time::Duration;
use bincode::error::{DecodeError, EncodeError};
use crossbeam_channel::{Receiver, Sender, SendError, TryRecvError};

use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;
use crate::cache::Data;
use crate::config::Config;


pub async fn main_network(data_receiver: Receiver<Data>, config_sender: Sender<Config>, restart_sender: Sender<bool>) {
    let local_addr = format!("{}:{}", "192.168.31.240", "9999");
    let mut send_buf: [u8; 65535] = [0; 65535];
    let mut recv_buf:[u8; 65535] = [0; 65535];
    let local = UdpSocket::bind(local_addr).await.expect("bind ip failed");
    loop {
        // config listener
        match local.try_recv_from(&mut recv_buf) {
            Ok(_) => {
                // decode
                let recv: String = match bincode::serde::decode_from_slice(&mut recv_buf, bincode::config::legacy()) {
                    Ok((recv, _usize)) => { /*println!("{:?}", recv);*/ recv}
                    Err(e) => { println!("decode string -> {:?}", e); continue }
                };
                let config: Config = match serde_json::from_str(recv.as_str()) {
                    Ok(config) => { config }
                    Err(e) => {println!("decode config json -> {:?}", e); continue }
                };
                // send
                match config_sender.try_send(config) {
                    Ok(_) => {/*println!("{:?}", recv)*/}
                    Err(_) => {} // try send no need to report error
                }
            }
            Err(_) => {} // try recv no need to report error
        };
        // data sender
        match data_receiver.try_recv() {
            Ok(data) => {
                // encode
                let data_json = match serde_json::to_string(&data) {
                    Ok(v) => { /*println!("{:?}", v);*/ v}
                    Err(e) => { println!("encode data json -> {:?}", e); continue }
                };
                let data_length = match bincode::serde::encode_into_slice(data_json, &mut send_buf, bincode::config::legacy()) {
                    Ok(v) => {/*println!("length -> {:?}", config_length);*/ v}
                    Err(e) => { println!("encode string -> {:?}", e); continue }
                };
                // send
                match local.try_send_to(&send_buf[..data_length], SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 31, 17)), 9998)) {
                    Ok(_) => { /*println!("{:?}", data)*/ }
                    Err(_) => {} // try send no need to report error
                };
            }
            Err(_) => {} // try recv no need to report error
        }
        sleep(Duration::from_millis(100));
    }
}