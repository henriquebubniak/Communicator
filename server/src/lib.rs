use eframe::egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};
use rsa::{Error, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde_yaml;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream, pub_key: &RsaPublicKey) -> Result<Vec<u8>, &'static str> {
    let mut data = [0 as u8; 500]; // using 500 byte buffer
    match stream.read(&mut data) {
        Ok(size) => {
            let s = serde_yaml::to_string(&pub_key).expect("failed to convert pub key to string");
            stream.write(s.as_bytes()).unwrap();
            let data_vec = data[0..size].to_vec();
            Ok(data_vec)
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap();
            Err("Could not read connection from ip {}")
        }
    }
}

// has pub_key as a parameter because the public key is sent as reply to every connection
pub fn expect_message(pub_key: &RsaPublicKey) -> Result<Vec<u8>, &'static str> {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server listening on port 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                drop(listener);
                return handle_client(stream, pub_key);
            }
            Err(_) => {
                drop(listener);
                return Err("Failed connection");
            }
        }
    }
    Err("No connection")
}

pub fn to_binary(bytes: &[u8]) -> Vec<u8> {
    let mut bin_bytes = String::new();
    for &byte in bytes {
        bin_bytes.push_str(&format!("{:b}", byte));
    }
    bin_bytes.into_bytes().iter().map(|c| *c - 48).collect()
}

pub fn decrypt_message(cyphertext: &Vec<u8>, key: &RsaPrivateKey) -> Result<String, Error> {
    match key.decrypt(Pkcs1v15Encrypt, cyphertext) {
        Ok(msg) => Ok(String::from_utf8(msg).unwrap()),
        Err(e) => Err(e),
    }
}

pub fn plot_message(msg: Vec<u8>, ui: &mut Ui) {
    let plot: PlotPoints = (0..msg.len())
        .map(|i| {
            let x = i as f64;
            [x, msg[i].into()]
        })
        .collect();
    let line = Line::new(plot);
    Plot::new("my_plot")
        .view_aspect(2.0)
        .show(ui, |plot_ui| plot_ui.line(line));
}
