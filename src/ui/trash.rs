use std::error::Error;
use log::{info, error};
use eframe::egui;
use crate::app::{App};

impl App {
    pub fn show_trash(&mut self, ui: &mut egui::Ui) 
        -> Result<(), Box<dyn Error>> {
        if !self.state_trash_load {
            let db = crate::db::database::Database::new(&self.db_path)?;
            match db.get_trash() {
                Ok(x) => {
                    self.notes_deleted = x;
                    self.state_trash_load = true; // TODO: move it to state
                }
                Err(e) => {
                    error!("Error loading notes from table note: {e}");
                    self.notes_deleted.clear();
                }
            }
        }

        if self.notes_deleted.is_empty() {
            ui.label("Trash is empty");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // for borrow issues
                let xs: Vec<(i64, String)> = self.notes_deleted.iter()
                    .map(|(id, name)| (*id, name.clone()))
                    .collect();
                for (id, name) in xs {
                    let selected = Some(&id) == self.selected_index.as_ref();

                    let response = ui.add(egui::SelectableLabel::new(selected, name));

                    if response.clicked() {
                        self.selected_index = Some(id);
                        // let _ = self.try_get_note(note.id);
                        println!("Trash note clicked {:?}", self.selected_index);
                    }

                    // right btn
                    response.context_menu(|ui| {
                        ui.set_min_width(120.0);
                        if ui.button("Restore").clicked() {
                            let _ = self.try_restore_note(id);
                            ui.close_menu();
                        }

                        if ui.button("Permanently Delete").clicked() {
                            info!("Delete clicked");
                            let _ = self.try_permanently_delete(id);
                            ui.close_menu();
                        }
                    });
                }
            });
        }
        Ok(()) 
    }

    fn try_restore_note(&mut self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        println!("id: {:?}", id);
        let mut db = crate::db::database::Database::new(&self.db_path)?;
        self.status_error = crate::utils::result(db.restore_note(id), "Error restoring note");
        //let _ = db.restore_note(id);

        // refresh ui
        self.load_rows = false;
        self.state_trash_load = false;
        Ok(())
    }
    
    fn try_permanently_delete(&mut self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        println!("id: {:?}", id);
        let mut db = crate::db::database::Database::new(&self.db_path)?;
        self.status_error = crate::utils::result(
            db.delete_note_hard(id),
            "Error deleting note");
        //let _ = db.delete_note_hard(id);

        // refresh ui
        self.load_rows = false;
        self.state_trash_load = false;
        Ok(())
    }
}
