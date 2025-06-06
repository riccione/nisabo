use eframe::egui::{self};
use log::{info, error};
use crate::app::{App};
use crate::constants::RESULT_SUCCESS;

impl App {
    pub fn show_rename(&mut self, ctx: &egui::Context) { 
        if self.state_rename {
            // tmp var
            let mut open = self.state_rename;
            egui::Window::new("Rename File")
                .open(&mut open)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Enter new name: ");
                    if let Some(e) = &self.rename_error {
                        ui.label(egui::RichText::new(e).color(egui::Color32::RED));
                    }

                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.rename_input);
                        
                        if self.rename_input.trim().is_empty() {
                            self.rename_error = Some("Name cannot be empty".to_string());
                        } else {
                            if ui.button("Rename").clicked() {
                                if let Err(e) = self.try_rename_note() {
                                    error!("Rename failed: {e}");
                                }
                            }
                        }
                  
                        if ui.button("Cancel").clicked() {
                            info!("Cancel clicked");
                            self.state_rename = false;
                            self.rename_target = None;
                            self.rename_input.clear();
                            self.rename_error = None;
                        }
                    });
                });
            if !open {
                self.state_rename = false;
                self.rename_target = None;
                self.rename_input.clear();
                self.rename_error = None;
            }
        }
    }

    fn try_rename_note(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = crate::db::database::Database::new(&self.db_path)?;
        self.status_error = match db.update_note_name(
            self.selected_index.unwrap(),
            &self.rename_input) {
            Ok(()) => String::from(RESULT_SUCCESS),
            Err(e) => format!("Error renaming note: {:?}", e),
        };

        self.state_rename = false;
        self.rename_target = None;
        self.rename_input.clear();
        self.rename_error = None;
        // refresh ui
        self.load_rows = false;
        Ok(())
    }
}
