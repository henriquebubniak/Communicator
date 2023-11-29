#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use server::{decrypt_message, expect_message, plot_message, to_binary};
use rand;
use rsa::{RsaPrivateKey, RsaPublicKey};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Server",
        options,
        Box::new(|_| Box::<MyApp>::default()),
    )
}

struct MyApp {
    message: Vec<u8>,
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
                String::from_utf8_lossy(&self.message)
            ));
            let binary_message = to_binary(&self.message);
            let binary_message_string: Vec<u8> = binary_message.iter().map(|i| *i + 48).collect();
            ui.label(format!(
                "As bytes: {}",
                String::from_utf8_lossy(&binary_message_string)
            ));
            if ui.button("Wait for message").clicked() {
                self.message = match expect_message(&self.pub_key) {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("{}", e);
                        vec![]
                    }
                };
            }
            if ui.button("Decrypt message").clicked() {
                self.decrypted_message = match decrypt_message(&self.message, &self.priv_key) {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("{}", e);
                        "".to_owned()
                    }
                }
            }
            ui.label(format!("Decrypted message: {}", self.decrypted_message));

            plot_message(binary_message, ui);
        });
    }
}
