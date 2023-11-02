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
    let local_addr = format!("{}:{}", "192.168.31.143", "9999");
    let mut send_buf: [u8; 10240] = [0; 10240];
    let mut recv_buf:[u8; 10240] = [0; 10240];
    let local = UdpSocket::bind(local_addr).await.expect("bind ip failed");
    loop {
        // config listener
        match local.try_recv_from(&mut recv_buf) {
            Ok(_) => {
                match bincode::serde::decode_from_slice(&mut recv_buf, bincode::config::legacy()) {
                    Ok((recv, _usize)) => {
                        match config_sender.try_send(recv) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                        // println!("{:?}", recv);
                    }
                    Err(_) => {}
                }


            }
            Err(_) => {}
        };
        // data sender
        match data_receiver.try_recv() {
            Ok(data) => {
                match bincode::serde::encode_into_slice(data.clone(), &mut send_buf, bincode::config::legacy()) {
                    Ok(config_length) => { println!("{:?}", config_length) }
                    Err(_) => {}
                };

                match local.try_send_to(&send_buf, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 31, 143)), 9998)) {
                    Ok(_) => { println!("{:?}", data)}
                    Err(_) => {}
                };
            }
            Err(_) => {}
        }
        sleep(Duration::from_millis(100));
    }
}