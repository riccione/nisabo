use std::path::{PathBuf};
use rfd::FileDialog;
use log::{info, error};
use crate::config::Config;
use std::error::Error;
use eframe::egui;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
// replace NoteIdName to Note
use crate::db::models::{NoteIdName, Note};
use crate::font::FontManager;
use crate::constants::{DEFAULT_FONT_DIR, DEFAULT_FONT};

#[derive(PartialEq)]
pub enum SidebarTab {
    Notes,
    Trash,
}

pub enum IoOperation {
    Import,
    Export,
}

#[derive(Debug, Default)]
pub enum ProgressState {
    #[default]
    Idle,               // not shown
    InProgress(f32),
    Completed(String),  // result
    Failed(String),    // error
}

#[derive(Default)]
pub struct App {
    pub db_path: String, // TODO: remove, use config.last... instead
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
    pub font_size: f32, // TODO: remove, use from config
    pub default_font_size: f32, // TODO: remove, use const from constans.rs
    pub config: Config,
    pub state_add_new_note: bool,
    pub parent_note_id: Option<i64>,
    pub add_new_note_input: String,
    pub add_new_note_error: Option<String>,
    pub original_content: String,
    pub edited_content: String,
    pub edited_note_id: Option<i64>,
    pub state_is_right_panel_on: bool,
    pub state_is_dark_mode: bool,

    pub import_done: Arc<AtomicBool>,
    pub state_exporting: bool,
    pub io_operation: Option<IoOperation>,
    
    pub state_io_progress: Option<f32>,
    pub state_importing: bool,
    pub io_rx: Option<std::sync::mpsc::Receiver<f32>>,

    pub state_progress: ProgressState,
    pub io_result: bool,
    pub io_status: String,

    pub names: Vec<NoteIdName>,
    pub status_error: String, // global error
    pub search_input: String,
    pub state_search: bool,
    pub search_result: Vec<Note>,
    pub search_has_focus: bool,
    pub current_font: String,
    pub font_manager: FontManager,

    pub state_history_open: bool,
}

impl Default for SidebarTab {
    fn default() -> Self {
        SidebarTab::Notes
    }
}

impl App {
    pub fn default_values() -> Self {
        let config = Config::load_config();
        // get font dir
        let font_dir = match config.font_dir_as_string() {
            Some(x) => x,
            None => DEFAULT_FONT_DIR.to_string(),
        };

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
            config,
            state_add_new_note: false,
            parent_note_id: None,
            add_new_note_input: String::new(),
            add_new_note_error: None,
            original_content: String::new(),
            edited_content: String::new(),
            edited_note_id: None,
            state_is_right_panel_on: true,
            state_is_dark_mode: true,

            import_done: Arc::new(AtomicBool::new(false)),
            state_exporting: false,
            io_operation: None,
            
            state_io_progress: None,
            state_importing: false,
            io_rx: None,

            state_progress: ProgressState::Idle,
            io_result: false,
            io_status: String::new(),

            names: Vec::<NoteIdName>::new(),
            status_error: String::new(),
            search_input: String::new(),
            state_search: false,
            search_result: Vec::<Note>::new(),
            search_has_focus: false,
            current_font: String::new(),
            font_manager: FontManager::new(font_dir),

            state_history_open: false,
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default_values();
        
        let config = Config::load_config();
        if let Some(x) = config.last_archive_path.clone() {
            if x.exists() {
                app.db_path = x.to_string_lossy().into_owned();
                app.state_start = true;
            }
        }
        
        // get font dir
        app.font_manager.font_dir = match config.font_dir_as_string() {
            Some(x) => x,
            None => DEFAULT_FONT_DIR.to_string(),
        };

        // load fonts
        app.font_manager.current_font = match config.font{
            Some(x) => x,
            None => String::from("Default"),
        };
        app.apply_font(&cc.egui_ctx);

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
                   
                    let font_dir = PathBuf::from(DEFAULT_FONT_DIR);
                  
                    let config = Config {
                        last_archive_path: Some(path.clone()),
                        font_dir: Some(font_dir.clone()),
                        font: Some(DEFAULT_FONT.to_string()),
                        font_size: self.font_size,
                        is_dark_mode: Some(self.state_is_dark_mode),
                        autosave: Some(true),
                    };
                    config.save_config();

                    // update values in Config struct 
                    self.config.last_archive_path = Some(path.clone());
                    self.config.font_dir = Some(font_dir);
                    self.config.font = Some(DEFAULT_FONT.to_string());
                    self.config.font_size = self.font_size;
                    self.config.is_dark_mode = Some(self.state_is_dark_mode);

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
            let config = Config {
                last_archive_path: Some(path.clone()),
                font_dir: Some(PathBuf::from(DEFAULT_FONT_DIR)),
                // TODO: need to replace with fm.current_font
                font: Some(DEFAULT_FONT.to_string()),
                font_size: self.font_size,
                is_dark_mode: Some(self.state_is_dark_mode),
                autosave: Some(true),
            };
            println!("last archive path: {:?}", config.last_archive_path);
            config.save_config();
            
            // update values in Config struct 
            self.config.last_archive_path = Some(path.clone());
            self.config.font_dir = Some(PathBuf::from(DEFAULT_FONT_DIR));
            self.config.font = Some(DEFAULT_FONT.to_string());
            self.config.font_size = self.font_size;
            self.config.is_dark_mode = Some(self.state_is_dark_mode);
            
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
    
    pub fn show_progress_window(
        &mut self,
        ctx: &egui::Context,
        title: &str,
        message: &str,
        progress: Option<f32>,
        show_close_btn: bool,
    ) {
        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(message);
                ui.label(&self.io_status);
                if let Some(progress) = progress {
                    ui.add(egui::ProgressBar::new(progress).show_percentage());
                }

                if show_close_btn {
                    if ui.button("Close").clicked() {
                        self.state_progress = ProgressState::Idle;
                        self.io_result = false;
                        self.io_status = String::new();
                        self.io_operation = None;
                    }
                }
            });
    }

    pub fn io_labels(&self) -> (&'static str, &'static str, &'static str) {
        match self.io_operation {
            Some(IoOperation::Import) => (
                "Importing notes..",
                "Import in progress. Please wait..",
                "Import failed",
            ),
            Some(IoOperation::Export) => (
                "Exporting notes..",
                "Export in progress. Please wait..",
                "Export failed",
            ),
            None => ("", "", ""),
        }
    }
}
