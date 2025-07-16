use eframe::egui::{self, Align, Layout, Color32};
use crate::app::{App};
use crate::ui::toggle_compact::toggle;

impl App {

    pub fn show_toolbar(&mut self, ctx: &egui::Context) {
        const ICON_ADD: egui::ImageSource<'_> = 
            egui::include_image!("../../assets/icons/plus-circle-1425-svgrepo-com.svg");
        const ICON_SAVE: egui::ImageSource<'_> = 
            egui::include_image!("../../assets/icons/save-item-1411-svgrepo-com.svg");
        const ICON_REFRESH: egui::ImageSource<'_> = 
            egui::include_image!("../../assets/icons/arrow-repeat-236-svgrepo-com.svg");

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let style = ui.style_mut();
            style.spacing.button_padding = [0.0; 2].into();
            style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
            style.visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;

            let tint = if self.state_is_dark_mode {
                Color32::WHITE
            } else {
                Color32::BLACK
            };

            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                if ui.add_sized(
                    [20.0, 20.0],
                    egui::ImageButton::new(ICON_ADD)
                        .tint(tint)
                ).clicked() {
                    self.state_add_new_note = true;
                }
                let save = ui.add_sized(
                    [20.0, 20.0],
                    egui::ImageButton::new(ICON_SAVE)
                        .tint(Color32::LIGHT_RED)
                );
                if save.clicked() {
                    let _ = self.try_update_note_content();
                }
                if ui.add_sized(
                    [20.0, 20.0],
                    egui::ImageButton::new(ICON_REFRESH)
                        .tint(tint)
                ).clicked() {
                    self.load_rows = false;
                }
                
                ui.add_space(5.0);

                toggle(ui, &mut self.state_is_right_panel_on);
                
                ui.add_space(5.0);

                let is_enabled = self.edited_note_id.is_some();
                if ui.add_enabled(is_enabled, egui::Button::new("History")).
                    clicked() {
                    self.state_history_open = true;
                };

            });
        });
    }
}
