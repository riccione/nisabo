use eframe::egui::{self};
use crate::app::{App};
use crate::constants::RESULT_SUCCESS;

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
                    // split history window to 2 panels
                    // left - show list of diffs
                    // right - show selected diff
                    // load list of diffs from the db
                    // add capability to select the diff and display it content
                    // inside the window
                    // add a button - Restore
                });
        });
        if !open {
            self.state_history_open = false;
        }
    }
}
