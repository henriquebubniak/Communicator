use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::error::Error;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

fn send_encrypted_message(msg: &str, stream: &mut TcpStream, pub_key:&RsaPublicKey) -> Result<usize, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let encrypted_msg = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes())
        .unwrap();
    let size = stream.write(&encrypted_msg)?;
    Ok(size)
}
fn send_non_encrypted_message(msg: &str, stream: &mut TcpStream) -> Result<usize, Box<dyn Error>> {
    let size = stream.write(msg.as_bytes())?;
    Ok(size)
}

pub fn get_rsa_pub_key(ip: &str) -> Option<RsaPublicKey> {
    match TcpStream::connect(ip) {
        Ok(mut stream) => {
            let _ = stream.write("get key".as_bytes());
            let mut data = [32 as u8; 1000]; // using 1000 byte buffer
            match stream.read(&mut data) {
                Ok(size) => {
                    let text = from_utf8(&data[0..size]).unwrap();
                    println!("Reply: {}", text);
                    let result: RsaPublicKey = serde_yaml::from_str(text).unwrap();
                    return Some(result);
                }
                Err(_) => {
                    println!("Failed to receive data");
                    return None;
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e.to_string());
            None
        }
    }
}

pub fn send_message(msg: &str, ip: &str, pub_key: &Option<RsaPublicKey>) -> Result<usize, Box<dyn Error>> {
    match TcpStream::connect(ip) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");
            match pub_key {
                Some(pub_key) => {
                    Ok(send_encrypted_message(msg, &mut stream, pub_key)?)
                }
                None => {
                    Ok(send_non_encrypted_message(msg, &mut stream)?)
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e.to_string());
            Err(Box::new(e))
        }
    }
}
