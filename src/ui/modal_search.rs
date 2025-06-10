use eframe::egui::{self, TextEdit, Label, RichText};
use log::{info, error};
use crate::app::{App};
use crate::constants::RESULT_SUCCESS;

impl App {
    pub fn show_search(&mut self, ctx: &egui::Context) { 
        let mut open = self.state_search;
        egui::Window::new("Search")
            .open(&mut open)
            .resizable(false)
            .anchor(egui::Align2::CENTER_TOP, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let response = ui.add(
                        TextEdit::singleline(&mut self.search_input)
                            .hint_text("Search")
                    );

                    if self.search_has_focus {
                        response.request_focus();
                        self.search_has_focus = false;
                    }

                    let search_input_empty = self.search_input.trim().is_empty();

                    let search_btn = ui.add_enabled(
                                        !search_input_empty, 
                                        egui::Button::new("Search")).clicked();

                    let enter_pressed = response.lost_focus() 
                        && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if (search_btn || enter_pressed) && !search_input_empty {
                        let _ = self.try_search();
                    }
                });

                if !self.search_result.is_empty() {
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([150.0, 0.0],
                                             Label::new(RichText::new("Name").strong()));
                                ui.add_sized([300.0, 0.0],
                                             Label::new(RichText::new("Content").strong()));
                            });
                                
                                for note in &self.search_result {
                                    ui.horizontal(|ui| {
                                        let preview = match &note.content {
                                            Some(text) => {
                                                let search = self.search_input.trim();
                                                
                                                text.split_whitespace()
                                                    .find(|word| word.to_lowercase().contains(&search))
                                                    .map(|w| w.to_string())
                                                    .unwrap_or_else(|| "No match".to_string())
                                                }, 
                                            None => String::from("No content"),
                                        };
                                        
                            ui.horizontal(|ui| {
                                ui.add_sized([150.0, 0.0],
                                             Label::new(&note.name));
                                ui.add_sized([300.0, 0.0],
                                             Label::new(preview));
                            });
                                    });
                                }
                        });
                }
        });
        if !open {
            self.state_search = false;
            self.search_input = String::new();
            self.search_has_focus = false;
        }
    }

    fn try_search(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let db = crate::db::database::Database::new(&self.db_path)?;
        match db.search(&self.search_input) {
            Ok(notes) => {
                self.search_result = notes;
                self.status_error = String::from(RESULT_SUCCESS);
            }
            Err(e) => {
                self.status_error = format!("Error searching note: {}", e);
            }
        }
        println!("{:?}", self.search_result);
        println!("{:?}", self.status_error);
        Ok(())
    }
}
