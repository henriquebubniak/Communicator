use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;

pub fn send_message(msg: &str, ip: &str) {
    match TcpStream::connect(ip) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            stream.write(msg.as_bytes()).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Reply: {}", text);
                },
                Err(_) => {
                    println!("Failed to receive data");
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e.to_string());
        }
    }
    println!("Terminated.");
}
