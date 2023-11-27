use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream)-> String {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    match stream.read(&mut data) {
        Ok(_size) => {
            // echo everything!
            stream.write(b"Ok").unwrap();
            let message = String::from_utf8_lossy(&data[..]);
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

pub fn expect_message() -> String {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                drop(listener);
                return handle_client(stream);
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