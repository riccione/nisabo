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
                        //let _ = self.show_db_ls(ui);
                        let _ = self.show_notes(ui);
                    },
                    SidebarTab::Trash => {
                        let _ = self.show_trash(ui);
                    }
                }
            });

        if self.state_is_right_panel_on {
            egui::SidePanel::right("right panel")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    render_md(ui, &self.edited_content);
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
