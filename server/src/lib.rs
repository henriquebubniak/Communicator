use eframe::egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};
use rsa::{Error, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde_yaml;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mlt3_decoding() {
        let msg = vec![1, 2, 2, 2, 1, 0, 1, 1, 1, 2, 1];
        let expected = vec![0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1];
        let encoded = decode_mlt3(&msg).unwrap();
        assert_eq!(encoded, expected)
    }

    #[test]
    fn test_from_binary() {
        let bin_msg = vec![0, 1, 1, 0, 0, 0, 0, 1];
        let bytes = from_binary(&bin_msg);
        let exp = vec![97];
        assert_eq!(bytes, exp);
    }
}

fn handle_client(mut stream: TcpStream, pub_key: &RsaPublicKey) -> Result<Vec<u8>, &'static str> {
    let mut data = [0 as u8; 20000]; // using 20000 byte buffer
    match stream.read(&mut data) {
        Ok(size) => {
            let s = serde_yaml::to_string(&pub_key).expect("failed to convert pub key to string");
            stream.write(s.as_bytes()).unwrap();
            let data_vec = data[0..size].to_vec();
            Ok(data_vec)
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap();
            Err("Could not read from connection")
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
pub fn decrypt_message(cyphertext: &Vec<u8>, key: &RsaPrivateKey) -> Result<String, Error> {
    match key.decrypt(Pkcs1v15Encrypt, cyphertext) {
        Ok(msg) => Ok(String::from_utf8(msg).unwrap()),
        Err(e) => Err(e),
    }
}

pub fn plot_message(msg: &Vec<u8>, ui: &mut Ui) {
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
