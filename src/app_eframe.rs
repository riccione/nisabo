use eframe::egui::{self, Button, Color32, RichText};
use log::{info};
use crate::ui::about::show_about;
use crate::app::{App, SidebarTab};
//use crate::ui::menu_bar;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let _archive_path = self.archive_path.clone();

        if self.state_rename {
            self.show_rename(ctx);
        }
        
        if self.show_settings {
            self.show_font_settings(ctx);
        }
        
        if self.state_add_new_note {
            self.show_add_new_note(ctx);
        }

        if self.state_start {
            self.show_menu_bar(ctx);    

            self.show_toolbar(ctx);
            
            egui::SidePanel::left("left panel")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        // Tabs
                        ui.horizontal(|ui| {
                            if ui.selectable_label(self.selected_tab == SidebarTab::Notes, "Notes").clicked() {
                                self.selected_tab = SidebarTab::Notes;
                                self.selected_index = None;
                                self.state_rename = false;
                            }
                            if ui.selectable_label(self.selected_tab == SidebarTab::Trash, "Trash").clicked() {
                                self.selected_tab = SidebarTab::Trash;
                                self.selected_index = None;
                                self.state_rename = false;
                            }
                        });

                        ui.separator();
                    });

                    // Tab content
                    match self.selected_tab {
                        SidebarTab::Notes => {
                            let _ = self.show_db_ls(ui);
                        },
                        SidebarTab::Trash => {
                            let _ = self.show_trash(ui);
                        }
                    }
                });

            egui::SidePanel::right("right panel")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    if self.edited_note.is_empty() {
                        ui.heading("Preview");
                    } else {
                        ui.label(self.edited_note.as_str());
                    }
                });

            egui::CentralPanel::default()
                .show(ctx, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.edited_note)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                    );
            });

            if self.show_about {
                show_about(ctx, &mut self.show_about);
            }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);

                    ui.label("Archive Name [optional]: ");
                    ui.text_edit_singleline(&mut self.archive_name);
                    
                    ui.add_space(20.0);

                    let softer_red = Color32::from_rgb(200, 50, 50);
                    let txt_create_archive = RichText::new("Create Archive")
                        .size(24.0)
                        .color(Color32::WHITE);
                    let btn_create_archive = Button::new(txt_create_archive)
                        .fill(softer_red);

                    if ui.add(btn_create_archive).clicked() {
                        info!("Create Archive clicked");
                        self.create_db();
                    }
                    
                    info!("{:?}", self.db_error);
                    if let Some(ref err) = self.db_error {
                        ui.colored_label(egui::Color32::RED, format!("{}", err));
                    }

                    ui.add_space(20.0);

                    if ui.button("Open Archive").clicked() {
                        info!("Open Archive clicked");
                        self.open_archive();
                    }
                });
            });
        }
    }
}
