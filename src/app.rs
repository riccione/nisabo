use std::path::{PathBuf};
use rfd::FileDialog;
use std::fs;
use log::{info, error};

#[derive(Default)]
pub struct App {
    pub archive_name: String,
    archive_path: Option<PathBuf>
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
}
