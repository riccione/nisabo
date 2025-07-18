use std::error::Error;
use log::{info, error};
use crate::db::models::{NoteIdName};
use eframe::egui::{self, RichText};
use crate::app::{App};

impl App {
    // TODO: refactor, same for trash
    pub fn show_notes(&mut self, ui: &mut egui::Ui) 
        -> Result<(), Box<dyn Error>> {
        if !self.load_rows {
            let db = crate::db::database::Database::new(&self.db_path)?;
            match db.get_notes() {
                Ok(notes) => {
                    self.names = notes;
                    self.load_rows = true; // TODO: move to state
                }
                Err(e) => {
                    error!("Error loading names from table archive: {e}");
                    self.names.clear();
                }
            }
        }

        if self.names.is_empty() {
            ui.label("No notes found");
        } else {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                self.draw_note_tree(ui);
            });
        }
        Ok(()) 
    }
    
    pub fn try_get_note(&mut self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let db = crate::db::database::Database::new(&self.db_path)?;
        let note = db.get_note(id)?;
        println!("{:?}", note);
        self.original_content = note.content.clone()
            .unwrap_or("".to_string());
        self.edited_content = note.content.unwrap_or("".to_string()); 
        self.edited_note_id = Some(id);
        Ok(())
    }
    
    fn try_delete_note(&mut self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        println!("id: {:?}", id);
        let mut db = crate::db::database::Database::new(&self.db_path)?;
        self.status_error = crate::utils::result(
            db.delete_note_and_children_soft(id),
            "Error deleting note");

        // refresh ui
        self.load_rows = false;
        self.state_trash_load = false;
        Ok(())
    }
    
    fn draw_note_tree(&mut self, ui: &mut egui::Ui) {
        //println!("{:?}", notes);
        for note in &self.names.clone() {
            self.draw_note(ui, note);
        }
    }

    fn draw_note(&mut self, ui: &mut egui::Ui, note: &NoteIdName) {
        let is_selected = Some(note.id) == self.selected_index;
        let display_name = if is_selected && self.edited_content != self.original_content {
            format!("* {}", note.name)
        } else {
            note.name.clone()
        };

        if note.children.is_empty() {
            let response = ui.add(egui::SelectableLabel::new(is_selected, &display_name));
            if response.clicked() {
                //self.selected_index = Some(note.id);
                //println!("Note selected: {}", note.id);
                // auto-save
                if self.edited_content != self.original_content {
                    let _ = self.try_auto_update_note_content();
                }

                // clear content after previously selected note
                self.edited_content = String::new();
                
                self.selected_index = Some(note.id);
                println!("note id {:?}", note.id);
                let _ = self.try_get_note(note.id);
            }

            // right btn selection
            if response.secondary_clicked() {
                self.selected_index = Some(note.id);
            }

            // right btn menu
            response.context_menu(|ui| {
                if !note.has_parent {
                    // add a child note, selected note is parent
                    if ui.button("Add child note").clicked() {
                        self.state_add_new_note = true;
                        // parent id
                        self.parent_note_id = Some(note.id);
                        ui.close_menu();
                    }
                }

                if ui.button("Rename").clicked() {
                    info!("Rename clicked with id: {}", note.id);
                    self.rename_input = note.name.to_string();
                    self.selected_index = Some(note.id);
                    // show popup with name as input
                    self.state_rename = true;
                    ui.close_menu();
                }

                if ui.button("Delete").clicked() {
                    info!("Delete clicked");
                    let _ = self.try_delete_note(note.id);
                    self.original_content = String::new();
                    self.edited_content = String::new();
                    ui.close_menu();
                }
            });
        } else {
            // get selection color from the theme
            let selection_color = ui.style().visuals.selection.bg_fill;

            let mut parent_text = RichText::new(&display_name);
            if is_selected {
                parent_text = RichText::new(&display_name)
                    .background_color(selection_color);
            }
            let header = egui::CollapsingHeader::new(parent_text)
                .default_open(false);

            let response = header.show(ui, |ui| {
                    for child in &note.children {
                        self.draw_note(ui, child);
                    }
            });

            if response.header_response.clicked() {
                //self.selected_index = Some(note.id);
                println!("Note selected: {}", note.id);
                // auto-save
                if !is_selected && self.edited_content != self.original_content {
                    let _ = self.try_auto_update_note_content();
                }

                // clear content after previously selected note
                self.edited_content = String::new();
                
                self.selected_index = Some(note.id);
                println!("note id {:?}", note.id);
                let _ = self.try_get_note(note.id);
            }

            response.header_response.context_menu(|ui| {
                // add a child note, selected note is parent
                if ui.button("Add child note").clicked() {
                    self.state_add_new_note = true;
                    // parent id
                    self.parent_note_id = Some(note.id);
                    ui.close_menu();
                }

                if ui.button("Rename").clicked() {
                    info!("Rename clicked with id: {}", note.id);
                    self.rename_input = note.name.to_string();
                    self.selected_index = Some(note.id);
                    // show popup with name as input
                    self.state_rename = true;
                    ui.close_menu();
                }

                if ui.button("Delete").clicked() {
                    info!("Delete clicked");
                    let _ = self.try_delete_note(note.id);
                    self.original_content = String::new();
                    self.edited_content = String::new();
                    ui.close_menu();
                }
            });
        }
    }

    pub fn should_save(&self) -> bool {
        self.original_content != self.edited_content
    }

    pub fn try_update_note_content(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(id) = self.selected_index && self.should_save() {
            let mut db = crate::db::database::Database::new(&self.db_path)?;
            match db.update_note_content(id, &self.edited_content) {
                Ok(_) => {
                    let _ = self.try_save_note_diff(id);
                    println!("Saved successfully!");
                    self.original_content = self.edited_content.clone();
                }
                Err(e) => println!("Failed to save: {e}"),
            } 
        }
        Ok(())
    }

    /*
     * autosave works only if 2 conditions met:
     * 1. self.config.autosave == true and
     * 2. self.edited_note_id.is_some()
     */ 
    pub fn try_auto_update_note_content(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let autosave = self.config.autosave.unwrap_or(true);
        if self.edited_note_id.is_some() && autosave && self.should_save() {
            let mut db = crate::db::database::Database::new(&self.db_path)?;
            match db.update_note_content(self.edited_note_id.unwrap(),  &self.edited_content) {
                Ok(_) => {
                    let _ = self.try_save_note_diff(self.edited_note_id.unwrap());
                    println!("Saved successfully!");
                    self.original_content = String::new(); 
                    self.edited_content = String::new();
                    self.edited_note_id = None;
                }
                Err(e) => println!("Failed to save: {e}"),
            } 
        }
        Ok(())
    }

    // draft
    fn try_save_note_diff(&mut self, note_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        /*
         * 1. create a diff using similar (self.original_content, self.edited_content)
         * 2. do serde -> needed for saving it in human readable format in db
         * 3. save diff to note_diff
         *
         *
         */ 
        let json = crate::diff::get_diff_json(
            &self.original_content,
            &self.edited_content);
        let mut db = crate::db::database::Database::new(&self.db_path)?;
        match db.insert_note_diff(note_id, &json) {
            Ok(_) => {
                println!("Note diff saved successfully!");
            }
            Err(e) => eprintln!("Failed to save note diff: {e}"),
        }
        Ok(())
    }
}
