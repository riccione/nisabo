use std::path::{PathBuf};
use rfd::FileDialog;
use eframe::egui;
use log::{info, error};
use crate::config::AppConfig;
use std::error::Error;

#[derive(PartialEq)]
pub enum SidebarTab {
    Notes,
    Trash,
}

#[derive(Default)]
pub struct App {
    pub db_path: String,
    pub show_about: bool,
    pub rename_target: Option<PathBuf>,
    pub rename_input: String,
    pub state_rename: bool,
    pub rename_error: Option<String>,
    pub db_error: Option<String>,
    pub load_rows: bool, // trigger loading
    pub names: Vec<(i32, String)>,
    pub notes_deleted: Vec<(i32, String)>,
    pub state_trash_load: bool, // trigger loading
    pub selected_index: Option<i32>,
    pub state_start: bool,
    pub selected_tab: SidebarTab,
    pub show_settings: bool,
    pub font_size: f32,
    pub default_font_size: f32,
    pub config: AppConfig,
    pub state_add_new_note: bool,
    pub add_new_note_input: String,
    pub add_new_note_error: Option<String>,
    pub original_content: String,
    pub edited_content: String,
    pub state_is_right_panel_on: bool,
    pub state_is_dark_mode: bool,
}

impl Default for SidebarTab {
    fn default() -> Self {
        SidebarTab::Notes
    }
}

impl App {
    pub fn default_values() -> Self {
        Self {
            db_path: String::new(),
            show_about: false,
            rename_target: None,
            rename_input: String::new(),
            state_rename: false,
            rename_error: None,
            db_error: None,
            load_rows: false,
            names: Vec::<(i32, String)>::new(),
            notes_deleted: Vec::<(i32, String)>::new(),
            state_trash_load: false,
            selected_index: None,
            state_start: false,
            selected_tab: SidebarTab::Notes,
            show_settings: false,
            font_size: 13.0,
            default_font_size: 13.0,
            config: AppConfig::load_config(),
            state_add_new_note: false,
            add_new_note_input: String::new(),
            add_new_note_error: None,
            original_content: String::new(),
            edited_content: String::new(),
            state_is_right_panel_on: true,
            state_is_dark_mode: true,
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default_values();
        
        let config = AppConfig::load_config();
        if let Some(x) = config.last_archive_path {
            if x.exists() {
                app.db_path = x.to_string_lossy().into_owned();
                app.state_start = true;
            }
        }
        
        app.font_size = if config.font_size < 1.0 { // without config.toml file
            app.default_font_size
        } else {
            config.font_size
        };
        app.apply_font_size(&cc.egui_ctx); 

        app.state_is_dark_mode = config.is_dark_mode.unwrap_or(true);

        app
    }

    pub fn create_db(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(path) = FileDialog::new()
            .set_title("Save your Archive")
            .set_file_name("archive.db")
            .save_file() {
            
            if path.try_exists()? {
                self.db_error = Some(format!("Database already exists at {:?}", 
                                             path));
            } else {
                if let Some(path_str) = path.to_str() {
                    let db = crate::db::database::Database::new(path_str)?;
                    db.init_tables()?;
                    
                    let config = AppConfig {
                        last_archive_path: Some(path.clone()),
                        font_size: self.font_size,
                        is_dark_mode: Some(self.state_is_dark_mode),
                    };
                    config.save_config();

                    self.state_start = true;
                    self.db_path = path.to_string_lossy().into_owned();
                    
                    self.load_rows = false;
                    // get rid of ghost data
                    self.selected_index = None;
                    self.original_content = String::new();
                    self.edited_content = String::new();
                } else {
                    self.db_error = Some("Path contains invalid UTF-8".to_string());
                }
            }
        } else {
            error!("No directory selected");
        }
        Ok(())
    }
    
    pub fn open_archive(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            info!("Archive opened from: {}", path.display());
            let config = AppConfig {
                last_archive_path: Some(path.clone()),
                font_size: self.font_size,
                is_dark_mode: Some(self.state_is_dark_mode),
            };
            config.save_config();
            
            let x = path.clone();
            self.db_path = x.to_string_lossy().into_owned();
            self.state_start = true;
            
            // get rid of ghost data
            self.selected_index = None;
            self.original_content = String::new();
            self.edited_content = String::new();
        } else {
            error!("No db file selected");
        }
    }
   
    // TODO: rename and refactor, same for trash
    pub fn show_db_ls(&mut self, ui: &mut egui::Ui) 
        -> Result<(), Box<dyn Error>> {
        if !self.load_rows {
            let db = crate::db::database::Database::new(&self.db_path)?;
            match db.get_notes() {
                Ok(names) => {
                    self.names = names;
                    self.load_rows = true; // TODO: move it to state
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                // for borrow issues
                let xs: Vec<(i32, String)> = self.names.iter()
                    .map(|(id, name)| (*id, name.clone()))
                    .collect();
                for (id, name) in xs {
                    let selected = Some(&id) == self.selected_index.as_ref();
                    
                    let mut display_name = name.clone();

                    if selected && self.edited_content != self.original_content {
                        let marker = "*";
                        println!("Unsaved change!");
                        display_name = format!("{marker} {display_name}");
                    }

                    let response = ui.add(egui::SelectableLabel::new(selected, display_name));
                    
                    if response.clicked() {
                        // clear content after previously selected note
                        self.edited_content = String::new();
                        self.selected_index = Some(id);
                        let _ = self.try_get_note(id);
                        println!("note clicked {:?}", self.selected_index);
                    }

                    // right btn
                    response.context_menu(|ui| {
                        if ui.button("Rename").clicked() {
                            info!("Rename clicked");
                            self.rename_input = name.to_string();
                            self.selected_index = Some(id);
                            // show popup with name as input
                            self.state_rename = true;
                            ui.close_menu();
                        }

                        if ui.button("Delete").clicked() {
                            info!("Delete clicked");
                            let _ = self.try_delete_note(id);
                            self.original_content = String::new();
                            self.edited_content = String::new();
                            ui.close_menu();
                        }
                    });
                }
            });
        }
        Ok(()) 
    }
    
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
                let xs: Vec<(i32, String)> = self.notes_deleted.iter()
                    .map(|(id, name)| (*id, name.clone()))
                    .collect();
                for (id, name) in xs {
                    let selected = Some(&id) == self.selected_index.as_ref();

                    let response = ui.add(egui::SelectableLabel::new(selected, name));

                    if response.clicked() {
                        self.selected_index = Some(id);
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
        let db = crate::db::database::Database::new(&self.db_path)?;
        println!("{:?}", self.selected_index);
        let _ = db.update_note_name(self.selected_index.unwrap(), &self.rename_input);

        self.state_rename = false;
        self.rename_target = None;
        self.rename_input.clear();
        self.rename_error = None;
        // refresh ui
        self.load_rows = false;
        Ok(())
    }
    
    fn try_delete_note(&mut self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        println!("id: {:?}", id);
        let db = crate::db::database::Database::new(&self.db_path)?;
        let _ = db.delete_note_soft(id);

        // refresh ui
        self.load_rows = false;
        self.state_trash_load = false;
        Ok(())
    }
    
    fn try_restore_note(&mut self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        println!("id: {:?}", id);
        let db = crate::db::database::Database::new(&self.db_path)?;
        let _ = db.restore_note(id);

        // refresh ui
        self.load_rows = false;
        self.state_trash_load = false;
        Ok(())
    }
    
    fn try_permanently_delete(&mut self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        println!("id: {:?}", id);
        let db = crate::db::database::Database::new(&self.db_path)?;
        let _ = db.delete_note_hard(id);

        // refresh ui
        self.load_rows = false;
        self.state_trash_load = false;
        Ok(())
    }
    
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
                            self.state_add_new_note = false;
                            self.add_new_note_input.clear();
                            self.add_new_note_error = None;
                        }
                    });
                });
            if !open {
                self.state_add_new_note = false;
                self.add_new_note_input.clear();
                self.add_new_note_error = None;
            }
        }
    }
    
    fn try_add_new_note(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let db = crate::db::database::Database::new(&self.db_path)?;
        let _ = db.add_new_note(&self.add_new_note_input);

        self.state_add_new_note = false;
        self.add_new_note_input.clear();
        self.add_new_note_error = None;
        // refresh ui
        self.load_rows = false;
        Ok(())
    }
    
    fn try_get_note(&mut self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = crate::db::database::Database::new(&self.db_path)?;
        let note = db.get_note(id)?;
        self.original_content = note.content.clone()
            .unwrap_or("".to_string());
        self.edited_content = note.content.unwrap_or("".to_string()); 
        Ok(())
    }
    
    pub fn try_update_note_content(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(id) = self.selected_index {
            let db = crate::db::database::Database::new(&self.db_path)?;
            match db.update_note_content(id, &self.edited_content) {
                Ok(_) => {
                    println!("Saved successfully!");
                    self.original_content = self.edited_content.clone();
                }
                Err(e) => println!("Failed to save: {e}"),
            } 
        }
        Ok(())
    }
}
