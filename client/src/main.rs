use eframe::egui;
use client::{send_non_encrypted_message, send_encrypted_message, get_rsa_pub_key, send_bits, plot_message, to_binary, encode_mlt3};
use rsa::RsaPublicKey;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Client",
        options,
        Box::new(|_| Box::<MyApp>::default()),
    )
}

struct MyApp {
    message: String,
    ip: String,
    pub_key: Option<RsaPublicKey>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            message: String::new(),
            ip: String::new(),
            pub_key: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Message sender");
            ui.horizontal(|ui| {
                let message_label = ui.label("Message: ");
                ui.text_edit_singleline(&mut self.message)
                    .labelled_by(message_label.id);
            });
            ui.horizontal(|ui| {
                let ip_label = ui.label("Ip: ");
                ui.text_edit_singleline(&mut self.ip)
                    .labelled_by(ip_label.id);
            });
            if ui.button("Send non encrypted").clicked() {
                match send_non_encrypted_message(&self.message, &self.ip) {
                    Ok(_) => { println!("Success"); },
                    Err(e) => { eprintln!("{}", e); }
                }
            }
            if ui.button("Send encrypted").clicked() {
                match send_encrypted_message(&self.message, &self.ip, &self.pub_key) {
                    Ok(_) => { println!("Success"); },
                    Err(e) => { eprintln!("{}", e); }
                }
            }
            if ui.button("Send bits").clicked() {
                match send_bits(&self.message, &self.ip) {
                    Ok(_) => { println!("Success"); },
                    Err(e) => { eprintln!("{}", e); }
                }
            }
            if ui.button("Get RSA public key").clicked() {
                self.pub_key = get_rsa_pub_key(&self.ip);
            }
            ui.label(format!("Message {}, sent to {}", self.message, self.ip));
            ui.label(format!("RSA Public key:\n {}", match &self.pub_key {
                Some(_) => "✅".to_owned(),
                None => "No public key".to_owned()
            }));
            let encoded = encode_mlt3(&to_binary(&self.message.as_bytes())).unwrap();
            plot_message(&encoded, ui);
        });
    }
}
