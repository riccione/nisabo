use eframe::egui::{self, Align, Layout, Color32};
use log::{info};
use crate::app::{App};

impl App {
    //const ICON_RIGHT: egui::ImageSource<'_> = egui::include_image!("../../assets/toggle-button-round-898-svgrepo-com.svg");

    pub fn show_toolbar(&mut self, ctx: &egui::Context) {
        const ICON_RIGHT: egui::ImageSource<'_> = egui::include_image!("../../assets/toggle-button-round-898-svgrepo-com.svg");

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        let style = ui.style_mut();
style.spacing.button_padding = [0.0; 2].into();
style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
style.visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
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
                let image = ICON_RIGHT;
                let response = ui.add_sized(
                    [24.0, 24.0],
                    egui::ImageButton::new(image)
                );
                if response.clicked() {
                    self.state_is_right_panel_visible = !self.state_is_right_panel_visible;
                }
            });
        });
    }
}
