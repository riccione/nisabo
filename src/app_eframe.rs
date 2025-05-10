use eframe::egui::{self, RichText, Color32, Button};
use log::{info};
use crate::app::App;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                ui.label("Archive Name [optional]: ");
                ui.text_edit_singleline(&mut self.archive_name);
                
                ui.add_space(20.0);

                let softer_red = Color32::from_rgb(200, 50, 50);
                let txt_create_archive = RichText::new("Create Archive")
                    .size(24.0)
                    .color(Color32::WHITE);
                let btn_create_archive = Button::new(txt_create_archive)
                    .fill(softer_red);

                if ui.add(btn_create_archive).clicked() {
                    info!("Create Archive clicked");
                    self.create_archive();
                }
                
                ui.add_space(20.0);

                if ui.button("Open Archive").clicked() {
                    info!("Open Archive clicked");
                }
            });
        });
    }
}
