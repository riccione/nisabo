use eframe::egui;
use crate::app::{App, SidebarTab};
use crate::markdown::render_md;

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
                        let _ = self.show_notes(ui);
                    },
                    SidebarTab::Trash => {
                        let _ = self.show_trash(ui);
                    }
                }
            });
        
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .show(ui, |ui| {
                        let total_width = ui.available_width();
                        let half_width = total_width / 2.0;

                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                if self.state_is_right_panel_on {
                                    ui.set_width(half_width);
                                }
                                if let Some(_) = self.selected_index {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut self.edited_content)
                                            .lock_focus(true)
                                            .desired_width(f32::INFINITY)
                                    );
                                }
                            });
                       
                            if self.state_is_right_panel_on {
                                ui.separator();

                                ui.vertical(|ui| {
                                    ui.set_width(half_width);
                                    render_md(ui, ctx, &self.edited_content);
                                });
                            }
                        });
            });
        });
    }
}
