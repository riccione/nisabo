use eframe::egui::{self};
use crate::app::{App};
use crate::constants::RESULT_SUCCESS;

impl App {
    pub fn show_history(&mut self, ctx: &egui::Context) {
        let mut open = self.state_history_open;
        self.history_ls.clear(); // clear history vector
        egui::Window::new("History")
            .open(&mut open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // left
                    ui.vertical(|ui| {
                        if let Some(id) = self.edited_note_id {
                            if let Err(e) = self.try_list_history(id) {
                                ui.label(format!("Error loading history: {}", e));
                            }
                            if !self.history_ls.is_empty() {
                                for x in &self.history_ls {
                                    ui.label(format!("{:?}", x));
                                }
                            }
                        } else {
                            ui.label("No note selected");
                        }
                        // ui.label(format!("{:?}", id));
                        // self.try_list_history(id);
                        // ui.label("History list");
                    });

                    ui.separator();

                    //right
                    ui.vertical(|ui| {
                        ui.label("Current diff");
                    });
                    // left - show list of diffs
                    // right - show selected diff
                    // load list of diffs from the db
                    // add capability to select the diff and display it content
                    // inside the window
                    // add a button - Restore
                });
        });
        if !open {
            self.state_history_open = false;
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
}
