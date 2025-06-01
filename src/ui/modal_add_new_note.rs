use eframe::egui::{self};
use log::{info, error};
use crate::app::{App};
use crate::db::models::{LinkType};

impl App {
    pub fn show_add_new_note(&mut self, ctx: &egui::Context) { 
        if self.state_add_new_note {
            // tmp var
            let mut open = self.state_add_new_note;
            egui::Window::new("Add new note")
                .open(&mut open)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Enter name: ");
                    if let Some(e) = &self.add_new_note_error {
                        ui.label(egui::RichText::new(e).color(egui::Color32::RED));
                    }

                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.add_new_note_input);
                        
                        if self.add_new_note_input.trim().is_empty() {
                            self.add_new_note_error = Some("Name cannot be empty".to_string());
                        } else {
                            if ui.button("Add").clicked() {
                                if let Err(e) = self.try_add_new_note() {
                                    error!("Add failed: {e}");
                                }
                            }
                        }
                  
                        if ui.button("Cancel").clicked() {
                            info!("Cancel clicked");
                            self.parent_note_id = None;
                            self.state_add_new_note = false;
                            self.add_new_note_input.clear();
                            self.add_new_note_error = None;
                        }
                    });
                });
            if !open {
                self.parent_note_id = None;
                self.state_add_new_note = false;
                self.add_new_note_input.clear();
                self.add_new_note_error = None;
            }
        }
    }
    
    fn try_add_new_note(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let db = crate::db::database::Database::new(&self.db_path)?;
        let target_id = db.add_new_note(&self.add_new_note_input)?;

        if let Some(pid) = self.parent_note_id {
            let _ = db.add_note_link(pid, target_id, LinkType::Parent);
        }

        self.parent_note_id = None;
        self.state_add_new_note = false;
        self.add_new_note_input.clear();
        self.add_new_note_error = None;
        // refresh ui
        self.load_rows = false;
        Ok(())
    }
}
