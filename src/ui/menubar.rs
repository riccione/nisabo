use eframe::egui;
use log::{info};
use crate::app::{App};

impl App {
    pub fn show_menubar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.horizontal(|ui| {
                ui.menu_button("Archive", |ui| {
                    if ui.button("Create").clicked() {
                        info!("Create clicked");
                        let _ = self.create_db();
                        ui.close_menu();
                    }
                    if ui.button("Open").clicked() {
                        self.open_archive();
                        self.load_rows = false;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Export to *.md").clicked() {
                        let _ = self.export("md");   
                        ui.close_menu();
                    }
                    if ui.button("Export to *.html").clicked() {
                        info!("Export to html clicked");
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Settings").clicked() {
                        info!("Settings");
                        self.show_settings = true;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        info!("About clicked");
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
                });
            });
        });
    }
}
