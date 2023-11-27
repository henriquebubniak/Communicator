#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_app::send_message;

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
    ip: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            message: String::new(),
            ip: String::new(),
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
            if ui.button("Send").clicked() {
                send_message(&self.message, &self.ip);
            }
            ui.label(format!("Message '{}', sent to {}", self.message, self.ip));
        });
    }
}