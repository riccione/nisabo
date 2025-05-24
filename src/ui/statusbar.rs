use eframe::egui;
use log::{info};
use crate::app::{App};

impl App {
    pub fn show_statusbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.db_path);
            });
        });
    }
}

