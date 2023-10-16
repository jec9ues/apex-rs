use std::thread::sleep;
use std::time::{Duration, Instant};
use egui_backend::egui::Pos2;
use serial2::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use serial2_tokio::SerialPort;

// calculate pitch -> -9.602106, yaw -> 72.99923
pub async fn main_kmbox_bpro(pitch: f32, yaw: f32, pitch_rate: f32, yaw_rate: f32) -> Result<(), Box<dyn std::error::Error>> {
    // println!("{}", format!("pitch, yaw({},{})\r\n", yaw, pitch));
    // println!("{}", format!("km.move({},{})\r\n", -(yaw * yaw_rate) as i16, (pitch * pitch_rate) as i16));
    let port = SerialPort::available_ports()?;
    let open_serial = SerialPort::open(&port[0], 115200)?;
    // 这里执行你的操作，比如发送串口数据
    // move 用来控制鼠标相对移动，输入参数(x,y)是数值类型。范围为-32767 到+32767。并且
    // 规定 x 向右为正，向左为负。Y 向下为正，向上为负。
    open_serial.write(format!("km.move({},{})\r\n", -(yaw * yaw_rate) as i16, (pitch * pitch_rate) as i16).as_bytes()).await?;
    // 等待一段时间，这是你在原代码中使用的 sleep 部分
    tokio::time::sleep(Duration::from_nanos(1)).await;
    Ok(())
}

