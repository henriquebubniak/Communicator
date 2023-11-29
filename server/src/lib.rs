use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use serde_yaml;

use rsa::RsaPublicKey;

fn handle_client(mut stream: TcpStream, pub_key: &RsaPublicKey)-> String {
    let mut data = [0 as u8; 500]; // using 50 byte buffer
    match stream.read(&mut data) {
        Ok(_size) => {
            let s = serde_yaml::to_string(&pub_key).expect("failed to convert to string");
            stream.write(s.as_bytes()).unwrap();
            let data: Vec<u8> = data.iter().map(|a| if *a == 0 { 32 } else { *a }).collect();
            let message = String::from_utf8_lossy(&data);
            let message = message.trim().to_owned();
            println!("Received: {}", message);
            format!("{}", message)
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            "error".to_owned()
        }
    }
}

pub fn expect_message(pub_key: &RsaPublicKey) -> String {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                drop(listener);
                return handle_client(stream, pub_key);
            }
            Err(e) => {
                /* connection failed */
                drop(listener);
                return format!("Error: {}", e);
            }
        }
    }
    "Error".to_owned()
}

pub fn to_binary(bytes: &[u8]) -> String {
    let mut bin_bytes = String::new();
    for &byte in bytes {
        bin_bytes.push_str(&format!("{:b}", byte));
    }
    bin_bytes
}