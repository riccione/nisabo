use eframe::egui::{self, TextEdit, Frame, Sense, Label, RichText, Layout, Align};
use log::{info, error};
use crate::app::{App};
use crate::constants::RESULT_SUCCESS;
use crate::ui::custom_button::left_aligned_button;

impl App {
    pub fn show_search(&mut self, ctx: &egui::Context) { 
        let mut open = self.state_search;
        egui::Window::new("Search")
            .open(&mut open)
            .resizable(false)
            //.anchor(egui::Align2::CENTER_TOP, [0.0, 0.0])
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
                    
                    let search = self.search_input.trim();
                    let mut results_to_display: Vec<(i64, String)> = Vec::new();

                    for note in &self.search_result {
                            let content = match &note.content {
                                Some(text) => {
                                    text.split_whitespace()
                                        .find(|word| word.to_lowercase().contains(&search))
                                        .map(|w| w.to_string())
                                        .unwrap_or_else(|| "No match".to_string())
                                    }, 
                                None => String::from("No content"),
                            };
                           
                            let name_match = note.name.to_lowercase().contains(&search);
                            let content_match = content.to_lowercase().contains(&search);
                           
                            if name_match || content_match {
                            let button_text = match (name_match, content_match) {
                                (true, true) => format!("{}: {}", note.name, content),
                                (true, false) => note.name.clone(),
                                (false, true) => content.clone(),
                                (false, false) => unreachable!(),
                            };

                            results_to_display.push((note.id, button_text));
                            }
                    }
                    if results_to_display.is_empty() {
                        ui.label("No match found");
                    } else {
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for (id, button_text) in &results_to_display {
                                ui.horizontal(|ui| {
                                    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                        if left_aligned_button(ui, button_text, 450.0, 24.0).clicked() {
                                            self.selected_index = Some(*id);
                                            let _ = self.try_get_note(*id);    
                                        }
                                    });
                                });
                            }
                        });
                    }
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
