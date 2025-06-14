use eframe::egui::{self, ComboBox};
use crate::app::{App};
use crate::ui::toggle_compact::toggle;
use crate::constants::{DEFAULT_IS_DARK_MODE, DEFAULT_FONT, DEFAULT_FONT_SIZE};

impl App {
    pub fn show_font_settings(&mut self, ctx: &egui::Context) {
        let mut open = self.show_settings;

        if self.show_settings {
            egui::Window::new("Settings")
                .collapsible(false)
                .resizable(false)
                .default_width(250.0)
                .open(&mut open) // toggles based on state
                .show(ctx, |ui| {
                    // dark/light mode
                    ui.label("Dark/Light mode:");
                    let response = toggle(ui, &mut self.state_is_dark_mode);
                    if response.changed() {
                        println!("{:?}", response);
                        self.config.is_dark_mode = Some(self.state_is_dark_mode); 
                        self.config.save_config();
                    }

                    ui.separator();
                    ui.label("Select font:");

                    let current_font = self.current_font.clone();
                    let fonts_ls = self.fonts_ls.clone();
                    ComboBox::from_label("Font")
                        .selected_text(&current_font)
                        .show_ui(ui, |ui| {
                            for font in &fonts_ls {
                                if ui.selectable_value(&mut self.current_font,
                                                       font.clone(),
                                                       font).clicked() {
                                    if self.current_font != "Default" {
                                        println!("apply font");
                                    }
                                    self.config.font = Some(self.current_font.clone());
                                    self.config.save_config();
                                }
                            }
                        });

                    ui.separator();

                    ui.label("Select font size:");

                    let font_sizes = vec![12.0, 13.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0];
                    let mut current_size = self.font_size;

                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:.1}", current_size))
                        .show_ui(ui, |ui| {
                            for &size in &font_sizes {
                                if ui
                                    .selectable_value(&mut current_size, size, format!("{size}"))
                                    .clicked()
                                {
                                    self.font_size = size;
                                    self.config.font_size = size;
                                    self.apply_font_size(ctx);
                                    self.config.save_config();
                                }
                            }
                        });

                    if ui.button("Reset to default").clicked() {
                        self.state_is_dark_mode = true;
                        self.current_font = DEFAULT_FONT.to_string();
                        self.font_size = DEFAULT_FONT_SIZE;
                        self.apply_font_size(ctx);

                        // reset values in the config
                        self.config.is_dark_mode = Some(DEFAULT_IS_DARK_MODE);
                        self.config.font = Some(DEFAULT_FONT.to_string());
                        self.config.font_size = DEFAULT_FONT_SIZE;
                        self.config.save_config();
                    }

                    ui.separator();

                    if ui.button("Close").clicked() {
                        self.show_settings = false;
                    }
                });
            if !open {
                self.show_settings = false;
            }
        }
    }

    pub fn apply_font_size(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(self.font_size + 6.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(self.font_size, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(self.font_size - 2.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Button, egui::FontId::new(self.font_size, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(self.font_size - 4.0, egui::FontFamily::Proportional)),
        ]
        .into();

        ctx.set_style(style);
    }
}
