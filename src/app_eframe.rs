use eframe::egui::{self, RichText, Color32, Button};
use log::{info};
use crate::ui::about::show_about;
use crate::app::{App, SidebarTab};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let _archive_path = self.archive_path.clone();

        if self.state_rename {
            self.show_rename(ctx);
        }

        if self.state_start {
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Archive", |ui| {
                        if ui.button("Create").clicked() {
                            info!("Create clicked");
                            ui.close_menu();
                        }
                        if ui.button("Open").clicked() {
                            info!("Open clicked");
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Help", |ui| {
                        if ui.button("About").clicked() {
                            info!("About clicked");
                            self.show_about = true;
                            ui.close_menu();
                        }
                    });
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
                            }
                            if ui.selectable_label(self.selected_tab == SidebarTab::Trash, "Trash").clicked() {
                                self.selected_tab = SidebarTab::Trash;
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
