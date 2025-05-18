use std::path::{PathBuf};
use rfd::FileDialog;
use std::fs;
use eframe::egui;
use log::{info, error};
use crate::config::AppConfig;
use std::error::Error;

#[derive(Default)]
pub struct App {
    pub archive_name: String,
    pub archive_path: Option<PathBuf>,
    pub db_path: String,
    pub selected_file: Option<PathBuf>,
    pub selected_file_content: Option<String>,
    pub show_about: bool,
    pub rename_target: Option<PathBuf>,
    pub rename_input: String,
    pub show_rename: bool,
    pub rename_error: Option<String>,
    pub db_error: Option<String>,
    pub load_rows: bool, // trigger loading
    pub names: Vec<(i32, String)>,
    pub selected_index: Option<i32>,
    pub state_start: bool,
}

impl App {
    pub fn default_values() -> Self {
        Self {
            archive_name: String::new(),
            archive_path: None,
            db_path: String::new(),
            selected_file: None,
            selected_file_content: None,
            show_about: false,
            rename_target: None,
            rename_input: String::new(),
            show_rename: false,
            rename_error: None,
            db_error: None,
            load_rows: false,
            names: Vec::<(i32, String)>::new(),
            selected_index: None,
            state_start: false,
        }
    }

    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default_values();
        
        let config = AppConfig::load_config();
        if let Some(x) = config.last_archive_path {
            if x.exists() {
                app.archive_path = Some(x.clone());
                app.db_path = x.to_string_lossy().into_owned();
                app.state_start = true;
            }
        }

        app
    }

    pub fn create_db(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(path) = FileDialog::new().pick_folder() {
            let archive_name = if self.archive_name.is_empty() {
                "archive"
            } else {
                &self.archive_name
            };

            let archive_path = path.join(format!("{}.db", archive_name));
            
            if archive_path.try_exists()? {
                self.db_error = Some(format!("Database already exists at {:?}", 
                                             archive_path));
            } else {
                if let Some(path_str) = archive_path.to_str() {
                    let db = crate::db::database::Database::new(path_str)?;
                    db.init_tables()?;
                    
                    let config = AppConfig {
                        last_archive_path: Some(archive_path.clone()),
                    };
                    config.save_config();

                    self.archive_path = Some(archive_path.clone());
                    self.state_start = true;
                    self.db_path = archive_path.to_string_lossy().into_owned();
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
            self.archive_path = Some(path.clone());
            let config = AppConfig {
                last_archive_path: self.archive_path.clone(),
            };
            config.save_config();
            
            let x = path.clone();
            self.db_path = x.to_string_lossy().into_owned();
            self.state_start = true;
        } else {
            error!("No db file selected");
        }
    }
    
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

        ui.heading("Notes list");

        if self.names.is_empty() {
            ui.label("No notes found");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (id, name) in &self.names {
                    let selected = Some(id) == self.selected_index.as_ref();
                    
                    if ui.add(egui::SelectableLabel::new(selected, name)).clicked() {
                        self.selected_index = Some(*id);
                        println!("note clicked {:?}", self.selected_index);
                    }
                }
            });
        }
        Ok(()) 
    }

    pub fn show_rename(&mut self, ctx: &egui::Context) {
        if self.show_rename {
            // tmp var
            let mut x = self.show_rename;
            egui::Window::new("Rename File")
                .open(&mut x)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Enter new name: ");
                    if let Some(e) = &self.rename_error {
                        ui.label(egui::RichText::new(e).color(egui::Color32::RED));
                    }

                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.rename_input);

                        if ui.button("Rename").clicked() {
                            if let Some(rename_target) = &self.rename_target {
                                let path = rename_target.clone();

                                let new_name = format!(
                                    "{}.{}",
                                    self.rename_input,
                                    path.extension()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                );

                                let new_path = path.with_file_name(new_name);

                                // 
                                if new_path.exists() {
                                    error!("The file already exists");
                                    self.rename_error = Some("The file with that name already exists".to_string());
                                    return; // keep popup open
                                } else {
                                    if let Err(e) = fs::rename(&path, &new_path) {
                                        error!("Error renaming file: {e}");
                                        self.rename_error = Some(format!("Failed to rename file: {e}"));
                                        return;
                                    } else {
                                        info!("File renamed to: {}", new_path.display());
                                        self.selected_file = Some(new_path.clone());
                                    }
                                }
                            }

                            self.show_rename = false;
                            self.rename_target = None;
                            self.rename_input.clear();
                            self.rename_error = None;
                        }
                     
                        if ui.button("Cancel").clicked() {
                            info!("Cancel clicked");
                            self.show_rename = false;
                            self.rename_target = None;
                            self.rename_input.clear();
                            self.rename_error = None;
                        }
                    });
                });
        }
    }
}
