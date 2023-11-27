#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use egui_server::{expect_message, to_binary};

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
    message: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            message: String::new(),
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
                self.message = expect_message();
            }
            let plot: PlotPoints = (0..self.message.len()).map(|i| {
                let x = i as f64;
                [x, (to_binary(self.message.as_bytes()).as_bytes()[i]-48).into()]
            }).collect();
            let line = Line::new(plot);
            Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
        });
    }
}
