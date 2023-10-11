
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::DecodePublicKey, Pkcs1v15Encrypt, pkcs8::DecodePrivateKey};

use serde::{Deserialize, Serialize};
use crate::constants::{CLIENT_PRIVATE_PEM, CLIENT_PUBLIC_PEM, SERVER_PUBLIC_PEM};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    key: Vec<u8>,
}

impl Data {
    pub fn new(value: Vec<u8>) -> Self{
        Data { key: value }
    }
}


#[tokio::main]
pub async fn verify_key() -> Result<String, Box<dyn std::error::Error>> {
    // 从命令行获取参数
    let args: Vec<String> = std::env::args().collect();

    // 检查是否提供了足够的参数
    if args.len() != 2 {
        eprintln!("Usage: {} <your_string>", args[0]);
        std::process::exit(1);
    }

    // 获取命令行参数中的字符串
    let input_string = &args[1];

    // 准备数据结构
    let enc_key = server_encode(input_string.as_bytes())?;
    let data = Data::new(enc_key);

    // 创建一个 reqwest 客户端
    let client = reqwest::Client::new();

    // 发送 POST 请求
    let uri = "https://api.ovo.rs/verify";
    let res = client.post(uri).json(&data).send().await?;

    // 检查响应状态码
    if res.status().is_success() {
        // 解析 JSON 响应
        let result: Data = res.json().await?;
        let status: String = String::from_utf8(client_decode(&result.key)?)?;
        println!("{:?}", status);
        if status == "valid" {
            return Ok(status);
        } else {
            return Err("Invalid status".into());
        }
    } else {
        eprintln!("Request failed with status code: {:?}", res.status());
        return Err("Request failed".into());
    }
}



pub fn server_encode(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    let public_key = RsaPublicKey::from_public_key_pem(SERVER_PUBLIC_PEM)?;

    match public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data) {
        Ok(data) => Ok(data),
        Err(err) => Err(Box::new(err)),
    }
}


pub fn client_encode(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    let public_key = RsaPublicKey::from_public_key_pem(CLIENT_PUBLIC_PEM)?;

    match public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data) {
        Ok(data) => Ok(data),
        Err(err) => Err(Box::new(err)),
    }
}
pub fn client_decode(data: &Vec<u8>)  -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let private_key = RsaPrivateKey::from_pkcs8_pem(CLIENT_PRIVATE_PEM)?;

    match private_key.decrypt(Pkcs1v15Encrypt, &data[..]) {
        Ok(data) => Ok(data),
        Err(err) => Err(Box::new(err)),
    }
}