use eframe::egui::{self, ComboBox};
use rfd::FileDialog;
use std::path::Path;
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
                        println!("VALUE: {:?}", self.config.last_archive_path);
                        self.config.is_dark_mode = Some(self.state_is_dark_mode); 
                        self.config.save_config();
                    }

                    ui.separator();
                    // fonts location
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(
                                &mut self.font_manager.font_dir));
                        
                        /*
                        let fd_response = ui.add(egui::TextEdit::singleline(
                                &mut self.font_manager.font_dir));
                        
                        // TODO: simplify logic
                        if fd_response.changed() {
                            // validate: check if path exists and it is a dir
                            let path = Path::new(self.font_manager.font_dir.trim());
                            if path.exists() && path.is_dir() {
                                // save to config
                                println!("Save to config from changed");
                            } else {
                                if !path.exists() {
                                    eprintln!("Error: path does not exist");
                                } else if !path.is_dir() {
                                    eprintln!("Error: path is not a directory");
                                } else {
                                    eprintln!("Ok");
                                }
                            }
                        }
                        */
                        if ui.button("...").clicked() {
                            if let Some(path) = FileDialog::new()
                                .set_title("Select dir with fonts")
                                .pick_folder() {
                                
                                self.font_manager.font_dir = path
                                    .to_string_lossy()
                                    .into_owned();
                                // save to config
                                self.config.font_dir = Some(path);
                                self.config.save_config();
                                
                                self.font_manager.load_available_fonts();
                            } else {
                                eprintln!("No directory selected");
                            }
                        } 
                    });

                    ui.separator();
                    ui.label("Select font:");

                    let current_font = self.font_manager.current_font.clone();
                    let fonts = self.font_manager.list_fonts().clone();
                    ComboBox::from_label("Font")
                        .selected_text(&current_font)
                        .show_ui(ui, |ui| {
                            for font in fonts {
                                if font == current_font {
                                    continue;
                                }
                                if ui.selectable_value(&mut self.font_manager.current_font,
                                                       font.clone(),
                                                       font).clicked() {
                                    if self.font_manager.current_font != "Default" {
                                        self.apply_font(ctx);
                                    }
                                    self.config.font = Some(self.font_manager.current_font.clone());
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

    pub fn apply_font(&mut self, ctx: &egui::Context) {
        let font_name = self.font_manager.current_font.clone();
        if font_name != DEFAULT_FONT {
            if let Some(font_data) = self.font_manager.get_font(&font_name) {
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert(
                    font_name.to_string(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data.clone())),
                );
            
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, font_name.to_string());
            
                ctx.set_fonts(fonts);
            }
        }
    }
}
