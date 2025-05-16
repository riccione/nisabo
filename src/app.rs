use std::path::{Path, PathBuf};
use rfd::FileDialog;
use std::fs;
use eframe::egui;
use std::io::Write;
use serde::{Deserialize, Serialize};
use log::{info, error};
use crate::config::AppConfig;
use rusqlite::{Connection, Result};
use chrono::{NaiveDateTime};

#[derive(Default)]
pub struct App {
    pub archive_name: String,
    pub archive_path: Option<PathBuf>,
    pub selected_file: Option<PathBuf>,
    pub selected_file_content: Option<String>,
    pub show_about: bool,
    pub rename_target: Option<PathBuf>,
    pub rename_input: String,
    pub show_rename: bool,
    pub rename_error: Option<String>,
    pub db_error: Option<String>,
}

#[derive(Default)]
struct Archive {
    id: i32,
    name: String,
    content: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl App {
    pub fn default_values() -> Self {
        Self {
            archive_name: String::new(),
            archive_path: None,
            selected_file: None,
            selected_file_content: None,
            show_about: false,
            rename_target: None,
            rename_input: String::new(),
            show_rename: false,
            rename_error: None,
            db_error: None,
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default_values();

        let config = AppConfig::load_config();
        if let Some(x) = config.last_archive_path {
            if x.exists() {
                app.archive_path = Some(x);
            }
        }

        app
    }

    pub fn create_db(&mut self) {
        if let Some(path) = FileDialog::new().pick_folder() {
            let archive_name = if self.archive_name.is_empty() {
                "archive"
            } else {
                &self.archive_name
            };

            let archive_path = path.join(format!("{}.db", archive_name));

            self.db_error = match crate::db::crud::create_db(&archive_path) {
                Ok(_) => None,
                Err(e) => Some(format!("Failed to create DB: {e}")),
            };
            info!("DB ERR: {:?}", self.db_error);
        }
    }

    pub fn open_archive(&mut self) {
        if let Some(path) = FileDialog::new().pick_folder() {
            info!("Archive opened from: {}", path.display());
            self.archive_path = Some(path);
            let config = AppConfig {
                last_archive_path: self.archive_path.clone(),
            };
            config.save_config();
        } else {
            error!("No directory selected");
        }
    }

    pub fn show_file_list(&mut self, ui: &mut egui::Ui, archive_path: &Path) {
        if let Ok(xs) = fs::read_dir(archive_path) {
            let mut files: Vec<_> = xs
                .filter_map(|entry| entry.ok())
                .filter(|e| e.path().is_file())
                .filter(|e| e.path()
                        .extension()
                        .and_then(|ext| ext.to_str()) == Some("md"))
                .collect();
            files.sort_by_key(|f| f.path());
            
            for f in files {
                let file_path = f.path();
                let _file_name = f.file_name().into_string().unwrap_or_default();
                let file_stem = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string();
                let response = ui.button(&file_stem);

                if response.clicked() {
                    info!("md file selected");
                    self.selected_file = Some(file_path.clone());
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        self.selected_file_content = Some(content.clone());
                    } else {
                        self.selected_file_content = None;
                    }
                }

                // right btn
                response.context_menu(|ui| {
                    if ui.button("Rename").clicked() {
                        info!("Rename clicked");
                        self.rename_target = Some(file_path.clone());
                        self.rename_input = file_stem.clone();
                        // TODO: show popup with a file stem
                        self.show_rename = true;
                        ui.close_menu();
                    }

                    if ui.button("Delete").clicked() {
                        info!("Delete clicked");
                        // TODO: handle delete
                    }
                });
            }
        } else {
            info!("Failed to read archive directory");
        }
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
