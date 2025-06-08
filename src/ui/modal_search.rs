use eframe::egui::{self, Align2, TextEdit};
use log::{info, error};
use crate::app::{App};
use crate::constants::RESULT_SUCCESS;

impl App {
    pub fn show_search(&mut self, ctx: &egui::Context) { 
        let mut open = self.state_search;
        egui::Window::new("Search")
            .open(&mut open)
            .resizable(false)
            .anchor(egui::Align2::CENTER_TOP, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_sized(
                    [100.0, 0.0],
                    TextEdit::singleline(&mut self.search_input)
                        .hint_text("Search")
                );

                ui.horizontal(|ui| {
                    ui.label("Search"); 
                });

        });
        if !open {
            self.state_search = false;
        }
    }
}
