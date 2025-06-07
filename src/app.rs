use std::path::{PathBuf};
use rfd::FileDialog;
use eframe::egui;
use log::{info, error};
use crate::config::AppConfig;
use std::error::Error;
use crate::db::models::{NoteIdName};

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
    pub notes_deleted: Vec<(i64, String)>,
    pub state_trash_load: bool, // trigger loading
    pub selected_index: Option<i64>,
    pub state_start: bool,
    pub selected_tab: SidebarTab,
    pub show_settings: bool,
    pub font_size: f32,
    pub default_font_size: f32,
    pub config: AppConfig,
    pub state_add_new_note: bool,
    pub parent_note_id: Option<i64>,
    pub add_new_note_input: String,
    pub add_new_note_error: Option<String>,
    pub original_content: String,
    pub edited_content: String,
    pub edited_note_id: Option<i64>,
    pub state_is_right_panel_on: bool,
    pub state_is_dark_mode: bool,
    pub state_export_progress: Option<f32>,
    pub state_exporting: bool,
    pub export_rx: Option<std::sync::mpsc::Receiver<f32>>,
    pub names: Vec<NoteIdName>,
    pub status_error: String,
    pub search_input: String,
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
            notes_deleted: Vec::<(i64, String)>::new(),
            state_trash_load: false,
            selected_index: None,
            state_start: false,
            selected_tab: SidebarTab::Notes,
            show_settings: false,
            font_size: 13.0,
            default_font_size: 13.0,
            config: AppConfig::load_config(),
            state_add_new_note: false,
            parent_note_id: None,
            add_new_note_input: String::new(),
            add_new_note_error: None,
            original_content: String::new(),
            edited_content: String::new(),
            edited_note_id: None,
            state_is_right_panel_on: true,
            state_is_dark_mode: true,
            state_export_progress: None,
            state_exporting: false,
            export_rx: None,
            names: Vec::<NoteIdName>::new(),
            status_error: String::new(),
            search_input: String::new(),
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
                    let mut db = crate::db::database::Database::new(path_str)?;
                    let _ = db.configure_db()?;
                    let _ = db.init_tables()?;
                    
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
  
    pub fn try_update_note_content(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(id) = self.selected_index {
            let mut db = crate::db::database::Database::new(&self.db_path)?;
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

    pub fn try_auto_update_note_content(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.edited_note_id.is_some() {
            let mut db = crate::db::database::Database::new(&self.db_path)?;
            match db.update_note_content(self.edited_note_id.unwrap(),  &self.edited_content) {
                Ok(_) => {
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

}
