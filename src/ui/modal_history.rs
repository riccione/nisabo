use eframe::egui::{self};
use crate::app::{App, LoadState};
use crate::db::models::{NoteDiff};
use crate::constants::RESULT_SUCCESS;
use crate::diff::{json_to_human};

impl App {
    pub fn show_history(&mut self, ctx: &egui::Context) {
        let mut open = self.state_history_open;
        egui::Window::new("History")
            .open(&mut open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let mut show_right_panel = false;
                    // left
                    ui.vertical(|ui| {
                        if let Some(id) = self.edited_note_id {
                            if self.history_ls.is_empty() && self.history_loaded_id != Some(id) {
                                if let Err(e) = self.try_list_history(id) {
                                    ui.label(format!("Error loading history: {}", e));
                                } else {
                                    self.history_loaded_id = Some(id); // it is loaded!
                                }
                            }
                            if !self.history_ls.is_empty() {
                                let mut selected_id = None;
                                show_right_panel = true;
                                for x in &self.history_ls {
                                    let f = format!("{}. {}", x.version, x.changed_at);
                                    if ui.button(f).clicked() {
                                        // println!("Clicked on diff: {}", x.id);
                                        selected_id = Some(x.id);
                                    };
                                }
                                if let Some(id) = selected_id {
                                    let _ = self.try_get_curr_history(id);
                                }
                            } else {
                                ui.label("No history");
                            }
                        } else {
                            ui.label("No note selected");
                        }
                    });

                    if show_right_panel {
                    ui.separator();

                    //right
                    ui.vertical(|ui| {
                        if self.history_curr != NoteDiff::default() {
                            let diff = json_to_human(&self.history_curr.diff);
                            ui.label(format!("Version: {}", self.history_curr.version));
                            ui.label(format!("Changed at: {}", self.history_curr.changed_at));
                            ui.label(format!("Diff:\n{}", diff));
                        }
                    });
                    }
                    // inside the window
                    // add a button - Restore
                });
        });
        if !open {
            self.state_history_open = false;
            self.history_ls.clear(); // clear history vector
            self.history_curr = NoteDiff::default();
            // TODO: add clear for data here
        }
    }

    fn try_list_history(&mut self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = crate::db::database::Database::new(&self.db_path)?;
        match db.select_note_diff_ls(id) {
            Ok(x) => {
                self.history_ls = x;
                self.status_error = String::from(RESULT_SUCCESS);
            }
            Err(e) => {
                self.status_error = format!("Error getting history: {}", e);
            }
        }
        Ok(())
    }
    
    fn try_get_curr_history(&mut self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = crate::db::database::Database::new(&self.db_path)?;
        match db.select_note_diff(id) {
            Ok(x) => {
                self.history_curr = x;
                self.status_error = String::from(RESULT_SUCCESS);
            }
            Err(e) => {
                self.status_error = format!("Error getting history: {}", e);
            }
        }
        Ok(())
    }
}
