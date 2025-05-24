use eframe::egui::{self, Align, Layout};
use log::{info};
use crate::app::{App};

impl App {
    pub fn show_toolbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                if ui.button("+").clicked() {
                    println!("Add button clicked!");
                    self.state_add_new_note = true;
                    ui.close_menu();
                }
                if ui.button("Save").clicked() {
                    println!("Save button clicked!");
                    let _ = self.try_update_note_content(); 
                }
                if ui.button("Right").clicked() {
                    self.state_is_right_panel_visible = !self.state_is_right_panel_visible;
                }
            });
        });
    }
}
