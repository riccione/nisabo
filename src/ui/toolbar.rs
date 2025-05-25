use eframe::egui::{self, Align, Layout, Color32};
use log::{info};
use crate::app::{App};
use crate::ui::toggle_compact::toggle;

impl App {

    pub fn show_toolbar(&mut self, ctx: &egui::Context) {
        const ICON_ADD: egui::ImageSource<'_> = 
            egui::include_image!("../../assets/plus-circle-1425-svgrepo-com.svg");
        const ICON_SAVE: egui::ImageSource<'_> = 
            egui::include_image!("../../assets/save-item-1411-svgrepo-com.svg");

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let style = ui.style_mut();
            style.spacing.button_padding = [0.0; 2].into();
            style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
            style.visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;

            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                if ui.add_sized(
                    [20.0, 20.0],
                    egui::ImageButton::new(ICON_ADD)
                ).clicked() {
                    self.state_add_new_note = true;
                }
                let save = ui.add_sized(
                    [20.0, 20.0],
                    egui::ImageButton::new(ICON_SAVE)
                );
                if save.clicked() {
                    let _ = self.try_update_note_content();
                }
                toggle(ui, &mut self.state_is_right_panel_on);
            });
        });
    }
    
}
