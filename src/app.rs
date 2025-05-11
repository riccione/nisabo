use std::path::{Path, PathBuf};
use rfd::FileDialog;
use std::fs;
use eframe::egui;
use log::{info, error, debug};

#[derive(Default)]
pub struct App {
    pub archive_name: String,
    pub archive_path: Option<PathBuf>
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            archive_name: String::new(),
            archive_path: None,
        };
        // customize egui with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals
        Self::default()
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
            }
        } else {
            error!("No directory selected");
        }
    }

    pub fn open_archive(&mut self) {
        if let Some(path) = FileDialog::new().pick_folder() {
            info!("Archive opened from: {}", path.display());
            self.archive_path = Some(path);
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
                let _response = ui.button(&file_stem);
            }
        } else {
            info!("Failed to read archive directory");
        }
    }
}
