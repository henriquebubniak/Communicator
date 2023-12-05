//use std::fs::File;
//use std::io::Write;
use eframe::egui;
use rand;
use rsa::{RsaPrivateKey, RsaPublicKey};
use server::{decode_mlt3, decrypt_message, expect_message, from_binary, plot_message};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native("Server", options, Box::new(|_| Box::<MyApp>::default()))
}

struct MyApp {
    message: Vec<u8>,         //mlt3 encoded
    decoded_message: Vec<u8>, //decoded, but binary
    bytes_message: Vec<u8>,   // bytes
    priv_key: RsaPrivateKey,
    pub_key: RsaPublicKey,
    decrypted_message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
        let pub_key = RsaPublicKey::from(&priv_key);
        Self {
            message: vec![],
            decoded_message: vec![],
            bytes_message: vec![],
            priv_key,
            pub_key,
            decrypted_message: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Message receiver");
            ui.label(format!(
                "Message: {}",
                String::from_utf8_lossy(&self.bytes_message)
            ));
            ui.label(format!("As bytes: {:?}", &self.decoded_message));
            if ui.button("Wait for message").clicked() {
                self.message = match expect_message(&self.pub_key) {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("{}", e);
                        vec![]
                    }
                };
                self.decoded_message = match decode_mlt3(&self.message) {
                    Ok(decoded) => decoded,
                    Err(e) => {
                        eprintln!("{}", e);
                        vec![]
                    }
                };
                self.bytes_message = from_binary(&self.decoded_message);

                //let mut file = File::create("./output").unwrap();
                //file.write_all(&self.decoded_message).unwrap();
            }
            if ui.button("Decrypt message").clicked() {
                self.decrypted_message =
                    match decrypt_message(&self.bytes_message, &self.priv_key) {
                        Ok(msg) => msg,
                        Err(e) => {
                            eprintln!("{}", e);
                            "".to_owned()
                        }
                    }
            }
            ui.label(format!("Decrypted message: {}", self.decrypted_message));

            plot_message(&self.message, ui);
        });
    }
}
