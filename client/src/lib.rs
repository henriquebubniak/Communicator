//use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::error::Error;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_mlt2() {
        let msg = vec![0,1,0,0,1,1,1,0,0,1,1];
        let encoded = encode_mlt3(&msg).unwrap();
        let expected = vec![1,2,2,2,1,0,1,1,1,2,1];
        assert_eq!(encoded,expected)
    }
    #[test]
    fn test_to_binary() {
        let msg = "a";
        let bin_msg = to_binary(msg.as_bytes());
        let exp = vec![0,1,1,0,0,0,0,1];
        assert_eq!(exp, bin_msg);
    }

    #[test]
    fn test_correctness() {
        use rand::Rng;
        for _ in 0..100 {
            let v_size: u32 = rand::thread_rng().gen_range(1..1000);
            let mut v = Vec::new();
            for _ in 0..v_size {
                let buf: u8 = rand::thread_rng().gen_range(0..=255);
                v.push(buf);
            }
            let binv = to_binary(&v);
            let encodev = encode_mlt3(&binv).unwrap();
            let decodev = decode_mlt3(&encodev).unwrap();
            let msg = from_binary(&decodev);
            assert_eq!(binv, decodev);
            assert_eq!(v, msg)
        }
        
    }
}

fn encode_mlt3(msg: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let mut last_transition: i8 = 1;
    let mut state: u8 = 1;
    let mut encoded = Vec::new();

    for bit in msg {
        if *bit == 1 {
            match state {
                1 => state = (state as i16 + last_transition as i16) as u8,
                2 => {
                    state = 1;
                    last_transition = -1;
                },
                0 => {
                    state = 1;
                    last_transition = 1;
                },
                _ => return Err("Invalid state")
            }
        }
        encoded.push(state)
    }
    Ok(encoded)
}

pub fn send_encrypted_message(msg: &str, ip: &str, pub_key: &Option<RsaPublicKey>) -> Result<usize, Box<dyn Error>> {
    match TcpStream::connect(ip) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");
            match pub_key {
                Some(pub_key) => {
                    let mut rng = rand::thread_rng();
                    let encrypted_msg = pub_key
                        .encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes())
                        .unwrap();
                    let bin_msg = to_binary(&encrypted_msg);
                    //let mut file = File::create("./output")?;
                    //file.write_all(&bin_msg)?;
                    let encoded = encode_mlt3(&bin_msg)?;
                    let size = stream.write(&encoded)?;
                    Ok(size)
                }
                None => {
                    println!("No RSA public key");
                    Err("No RSA public key".into())
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e.to_string());
            Err(Box::new(e))
        }
    }
}
pub fn send_non_encrypted_message(msg: &str, ip: &str) -> Result<usize, Box<dyn Error>> {
    match TcpStream::connect(ip) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");
            let bin_msg = to_binary(msg.as_bytes());
            let encoded = encode_mlt3(&bin_msg)?;
            let size = stream.write(&encoded)?;
            Ok(size)
        }
        Err(e) => {
            println!("Failed to connect: {}", e.to_string());
            Err(Box::new(e))
        }
    }
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

pub fn to_binary(bytes: &[u8]) -> Vec<u8> {
    let mut bin_bytes = String::new();
    for &byte in bytes {
        bin_bytes.push_str(&format!("{:08b}", byte));
    }
    bin_bytes.into_bytes().iter().map(|c|*c-48).collect()
}


pub fn from_binary(bin_bytes: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut buf = 0;
    for i in (0..bin_bytes.len()).rev() {
        buf += bin_bytes[i] * 2_u8.pow(((bin_bytes.len() - i - 1) % 8).try_into().unwrap());
        if i % 8 == 0 {
            bytes.push(buf);
            buf = 0;
        }
    }
    bytes.reverse();
    bytes
}


pub fn decode_mlt3(msg: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let mut decoded = Vec::new();
    let mut last_state = 1;
    for state in msg {
        if *state != last_state {
            decoded.push(1);
            last_state = *state;
        } else {
            decoded.push(0);
        }
    }
    Ok(decoded)
}