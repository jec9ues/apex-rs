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
    let ip = "192.168.31.143";
    let port = "9998";
    let mut send_buf: [u8; 10240] = [0; 10240];
    let mut recv_buf:[u8; 10240] = [0; 10240];
    let local = UdpSocket::bind(format!("{ip}:{port}")).await.expect("bind ip failed");
    loop {
        match local.try_recv_from(&mut recv_buf) {
            Ok(_) => {
                match bincode::serde::decode_from_slice(&mut recv_buf, bincode::config::legacy()) {
                    Ok((recv, _bytes_read)) => {
                        match data_sender.try_send(recv) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
                // println!("{:?}", recv);
            }
            Err(_) => {}
        };
        match config_receiver.try_recv() {
            Ok(config) => {
                match bincode::serde::encode_into_slice(config, &mut send_buf, bincode::config::legacy()) {
                    Ok(_config_length) => {/*println!("length -> {:?}", config_length)*/}
                    Err(_) => {}
                }
                match local.try_send_to(&send_buf, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 31, 143)), 9999)) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}