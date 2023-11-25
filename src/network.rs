use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread::sleep;
use std::time::Duration;
use bincode::error::{DecodeError, EncodeError};
use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError};

use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;
use crate::cache::Data;
use crate::config::Config;


pub async fn recv_main(data_sender: Sender<Data>, config_receiver: Receiver<Config>) {
    let ip = "192.168.31.17";
    let port = "9998";
    let mut send_buf: [u8; 65535] = [0; 65535];
    let mut recv_buf:[u8; 65535] = [0; 65535];
    let local = UdpSocket::bind(format!("{ip}:{port}")).await.expect("bind ip failed");
    loop {
        match local.try_recv_from(&mut recv_buf) {
            Ok((length, _remote_addr)) => {
                // decode
                let recv: String = match bincode::serde::decode_from_slice(&mut recv_buf[..length], bincode::config::legacy()) {
                    Ok((recv, _bytes_read)) => { recv }
                    Err(e) => { println!("decode string -> {:?}", e); continue }
                };
                let data: Data = match serde_json::from_str(recv.as_str()) {
                    Ok(data) => { data }
                    Err(e) => {println!("decode data json -> {:?}", e); continue }
                };
                // send
                match data_sender.try_send(data) {
                    Ok(_) => {/*println!("{length}");*/}
                    Err(_) => {} // try send no need to report error
                }
            }
            Err(_) => {} // try recv no need to report error
        };

        match config_receiver.try_recv() {
            Ok(config) => {
                // encode
                let config_json = match serde_json::to_string(&config) {
                    Ok(v) => { /*println!("{:?}", v);*/ v}
                    Err(e) => { println!("encode config json -> {:?}", e); continue }
                };
                let config_length = match bincode::serde::encode_into_slice(config_json, &mut send_buf, bincode::config::legacy()) {
                    Ok(v) => {/*println!("length -> {:?}", config_length);*/ v}
                    Err(e) => { println!("encode string -> {:?}", e); continue }
                };
                // send
                match local.try_send_to(&send_buf[..config_length], SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 31, 240)), 9999)) {
                    Ok(_) => {}
                    Err(_) => {}
                };
            }
            Err(_) => {}
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}