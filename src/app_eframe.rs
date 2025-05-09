use eframe::egui;
use crate::app::App;
use crate::APP_NAME;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(APP_NAME);
        });
    }
}
