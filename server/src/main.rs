#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use egui_server::{expect_message, to_binary};
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use rand;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| {
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    message: String,
    priv_key: RsaPrivateKey,
    pub_key: RsaPublicKey,
    decrypted_message: String
}

impl Default for MyApp {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
        let pub_key = RsaPublicKey::from(&priv_key);
        Self {
            message: String::new(),
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
            ui.label(format!("Message: {}", self.message));
            ui.label(format!("As bytes: {}", to_binary(self.message.as_bytes())));
            if ui.button("Wait for message").clicked() {
                self.message = expect_message(&self.pub_key);
            }
            if ui.button("Decrypt message").clicked() {
                let m = match self.priv_key.decrypt(Pkcs1v15Encrypt, self.message.as_bytes()){
                    Ok(mess) => mess,
                    Err(_) => {println!("Failed to decrypt"); Vec::new()}
                };
                self.decrypted_message = String::from_utf8(m).unwrap();
            }
            ui.label(format!("Decrypted message: {}", self.decrypted_message));
            let plot: PlotPoints = (0..self.message.len()).map(|i| {
                let x = i as f64;
                [x, (to_binary(self.message.as_bytes()).as_bytes()[i]-48).into()]
            }).collect();
            let line = Line::new(plot);
            Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
        });
    }
}
