use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;
use serde_yaml;

use rsa::RsaPublicKey;

fn handle_client(mut stream: TcpStream, pub_key: &RsaPublicKey)-> Vec<u8> {
    let mut data = [0 as u8; 500]; // using 50 byte buffer
    match stream.read(&mut data) {
        Ok(_size) => {
            let s = serde_yaml::to_string(&pub_key).expect("failed to convert to string");
            stream.write(s.as_bytes()).unwrap();
            let mut data_vec = Vec::new();
            let mut index = 499;
            for i in (0..500).rev() {
                if data[i] != 0 {
                    println!("Data: {}", data[i]);
                    index = i+1;
                    break;
                }
            }
            for i in 0..index {
                data_vec.push(data[i]);
            }
            println!("index: {index}");
            let mut file = File::create("./encrypted_received.yaml").expect("Failed to create file");
            file.write_all(&data_vec).expect("Failed to write file");
            println!("Received: {:?}", data_vec);
            data_vec
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            vec![]
        }
    }
}

pub fn expect_message(pub_key: &RsaPublicKey) -> Vec<u8> {
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
            Err(_) => {
                /* connection failed */
                drop(listener);
                return vec![];
            }
        }
    }
    vec![]
}

pub fn to_binary(bytes: &[u8]) -> String {
    let mut bin_bytes = String::new();
    for &byte in bytes {
        bin_bytes.push_str(&format!("{:b}", byte));
    }
    bin_bytes
}