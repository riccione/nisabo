use eframe::egui::{self, Align, Button, Color32, Layout, RichText};
use log::{info};
use crate::ui::about::show_about;
use crate::app::{App, SidebarTab};
use crate::ui::menu_bar;

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
            
            egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    if ui.button("+").clicked() {
                        println!("Add button clicked!");
                        self.state_add_new_note = true;
                        ui.close_menu();
                    }
                });
            });

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
                    if let Some(content) = &self.selected_file_content {
                        ui.heading("TODO: File Stem");
                        ui.separator();
                        // TODO: add a separate fn - for now just a stub
                        ui.label(content);
                    }
                });

            egui::CentralPanel::default()
                .show(ctx, |ui| {
                    if let Some(content) = &self.selected_file_content {
                        ui.heading("TODO: File Stem");
                        ui.separator();
                        // TODO: add a separate fn - for now just a stub
                        ui.label(content);
                    }
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
