use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::fs::File;

use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

pub fn send_message(msg: &str, ip: &str, pub_key: &Option<RsaPublicKey>) -> Option<RsaPublicKey> {
    match TcpStream::connect(ip) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");
            match pub_key {
                Some(pub_key) => {
                    let mut rng = rand::thread_rng();
                    let encrypted_msg = pub_key
                        .encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes())
                        .unwrap();
                    let encrypted_msg_bytes: &[u8] = &encrypted_msg;
                    let mut file = File::create("./encrypted_sent.yaml").expect("Failed to create file");
                    file.write_all(encrypted_msg_bytes).expect("Failed to write file");
                    stream.write(encrypted_msg_bytes).unwrap();
                    println!("Sent ENCRYPTED hello, awaiting reply...");

                    let mut data = [32 as u8; 1000]; // using 6 byte buffer
                    match stream.read(&mut data) {
                        Ok(_) => {
                            let text = from_utf8(&data).unwrap();
                            println!("Reply: {}", text);
                            let result: RsaPublicKey = serde_yaml::from_str(text).unwrap();
                            return Some(result);
                        }
                        Err(_) => {
                            println!("Failed to receive data");
                            return None;
                        }
                    }
                }
                None => {
                    stream.write(msg.as_bytes()).unwrap();
                    println!("Sent NOT ENCRYPTED hello, awaiting reply...");

                    let mut data = [0 as u8; 10000]; // using 6 byte buffer
                    match stream.read(&mut data) {
                        Ok(_) => {
                            let data: Vec<u8> = data.iter().map(|a| if *a == 0 { 32 } else { *a }).collect();
                            let data: &[u8] = &data;
                            let text = from_utf8(&data).unwrap().trim();
                            let mut file = File::create("./output.yaml").expect("Failed to create file");
                            file.write_all(text.as_bytes()).expect("Failed to write file");
                            println!("Reply: {}", text);
                            let result: RsaPublicKey = serde_yaml::from_str(text).unwrap();
                            return Some(result);
                        }
                        Err(_) => {
                            println!("Failed to receive data");
                            return None;
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e.to_string());
            return None;
        }
    }
}
