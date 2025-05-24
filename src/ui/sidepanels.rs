use eframe::egui::{self, Align, Layout};
use log::{info};
use crate::app::{App, SidebarTab};

impl App {
    pub fn show_sidepanels(&mut self, ctx: &egui::Context) {
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

        if self.state_is_right_panel_visible {
            egui::SidePanel::right("right panel")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    if self.edited_content.is_empty() {
                        ui.heading("Preview");
                    } else {
                        ui.label(self.edited_content.as_str());
                    }
                });
        }

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        if let Some(_) = self.selected_index {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.edited_content)
                                    .lock_focus(true)
                                    .desired_width(f32::INFINITY)
                            );
                        }
                    });
        });
    }
}
