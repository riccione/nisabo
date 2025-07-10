use eframe::egui::{self, TextEdit, Frame, Sense, Label, RichText, Layout, Align};
use log::{info, error};
use crate::app::{App};
use crate::constants::RESULT_SUCCESS;
use crate::ui::custom_button::left_aligned_button;

impl App {
    pub fn show_history(&mut self, ctx: &egui::Context) {
        let mut open = self.state_history_open;
        egui::Window::new("History")
            .open(&mut open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let id = self.edited_note_id;
                    ui.label(format!("{:?}", id));
                });
        });
        if !open {
            self.state_history_open = false;
        }
    }
}
