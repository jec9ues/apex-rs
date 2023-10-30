use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender, TryRecvError};

use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;
use crate::cache::Data;
use crate::config::Config;


pub async fn recv_main(data_sender: Sender<Data>, config_receiver: Receiver<Config>) {
    let ip = "192.168.31.143";
    let port = "9999";
    let mut send_buf: [u8; 1024] = [0; 1024];
    let mut recv_buf:[u8; 1024] = [0; 1024];
    let local = UdpSocket::bind(format!("{ip}:{port}")).await.expect("bind ip failed");
    loop {
        match local.try_recv_from(&mut recv_buf) {
            Ok(_) => {
                let (recv, _bytes_read): (Data, usize) = bincode::serde::decode_from_slice(&mut recv_buf, bincode::config::legacy()).unwrap();
                data_sender.send(recv).unwrap();
                // println!("{:?}", recv);
            }
            Err(_) => {}
        };
        match config_receiver.try_recv() {
            Ok(config) => {
                let config_length = bincode::serde::encode_into_slice(config, &mut send_buf, bincode::config::legacy()).unwrap();
                println!("{:?}", config_length);
                local.send_to(&send_buf, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 31, 143)), 9998)).await.unwrap();
            }
            Err(_) => {}
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}