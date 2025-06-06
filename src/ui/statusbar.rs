use eframe::egui::{self, Color32, RichText};
use crate::app::{App};

impl App {
    pub fn show_statusbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.db_path);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&self.status_error)
                            .color(Color32::from_rgb(200,0,0))
                            .strong()
                    );
                });
            });
        });
    }
}
