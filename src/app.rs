use std::path::{Path, PathBuf};
use rfd::FileDialog;
use std::fs;
use eframe::egui;
use std::io::Write;
use serde::{Deserialize, Serialize};
use log::{info, error};

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub last_archive_path: Option<PathBuf>,
}

#[derive(Default)]
pub struct App {
    pub archive_name: String,
    pub archive_path: Option<PathBuf>,
    pub selected_file: Option<PathBuf>,
    pub selected_file_content: Option<String>,
    pub show_about: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            archive_name: String::new(),
            archive_path: None,
            selected_file: None,
            selected_file_content: None,
            show_about: false
        };
        
        let config = Self::load_config();
        if let Some(x) = config.last_archive_path {
            if x.exists() {
                app.archive_path = Some(x);
            }
        }

        app
    }
    
    fn get_config_path() -> Option<PathBuf> {
        dirs::config_dir()
            .map(|dir| dir.join("nisabo/config.toml"))
    }

    fn load_config() -> AppConfig {
        info!("loading config");
        if let Some(config_path) = Self::get_config_path() {
            info!("{:?}", config_path);
            if let Ok(data) = fs::read_to_string(config_path) {
                if let Ok(config) = toml::from_str::<AppConfig>(&data) {
                    return config;
                }
            }
        }
        AppConfig::default()
    }

    fn save_config(&self) {
        info!("saving config");
        if let Some(config_path) = Self::get_config_path() {
            if let Some(parent) = config_path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            let config = AppConfig {
                last_archive_path: self.archive_path.clone(),
            };

            if let Ok(toml_str) = toml::to_string_pretty(&config) {
                let _ = fs::File::create(&config_path)
                    .and_then(|mut f| f.write_all(toml_str.as_bytes()));
            }
        }
    }

    pub fn create_archive(&mut self) {
        if let Some(path) = FileDialog::new().pick_folder() {
            let archive_name = if self.archive_name.is_empty() {
                "MyArchive"
            } else {
                &self.archive_name
            };
                
            let archive_path = path.join(archive_name);

            // create a directory (archive)
            if let Err(e) = fs::create_dir_all(&archive_path) {
                error!("Failed to create archive directory: {}", e);
            } else {
                // create a README.md file inside the empty archive
                let readme_path = archive_path.join("README.md");
                let default_content = "# Welcome to your Archive\n";

                match fs::write(&readme_path, default_content) {
                    Ok(_) => println!("README.md created at: {}", readme_path.display()),
                    Err(e) => eprintln!("Failed to create README.md: {}", e),
                }

                info!("Archive created at: {}", archive_path.display());

                self.archive_path = Some(archive_path);
                self.save_config();
            }
        } else {
            error!("No directory selected");
        }
    }

    pub fn open_archive(&mut self) {
        if let Some(path) = FileDialog::new().pick_folder() {
            info!("Archive opened from: {}", path.display());
            self.archive_path = Some(path);
            self.save_config();
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
            }
        } else {
            info!("Failed to read archive directory");
        }
    }

    pub fn mod_show_about(&mut self, ctx: &egui::Context, show: &mut bool) {
        info!("Open show about mod");
        egui::Window::new("About")
            .collapsible(false)
            .resizable(false)
            .open(show)
            .show(ctx, |ui| {
                ui.label("nisabo");
                ui.label("Version: 0.1.0");
                ui.hyperlink("https://");
            });
    }
}
