use eframe::egui;
use rfd::FileDialog;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use std::error::Error;
use log::{info, error};
use crate::app::{App};

impl App {
    pub fn export(&mut self, target: &str) -> Result<(), Box<dyn Error>> {
        if let Some(path) = FileDialog::new()
            .pick_folder() {
                
                // create an empty dir 'exported'
                let full_path = Path::new(&path).join("exported");
                fs::create_dir_all(&full_path)?;
                println!("Dir created at: {:?}", full_path);
                if target == "md" {
                let db = crate::db::database::Database::new(&self.db_path)?;
                let notes = db.get_all_notes()?;
                
                    for note in notes {
                        let safe_name = self.sanitize(&note.name);
                        let file_path = full_path.join(format!("{safe_name}.md"));

                        let mut file = File::create(&file_path)?;
                        
                        writeln!(file, "# {}", note.name)?;
                        writeln!(file)?;
                        writeln!(file, "{}", note.content.unwrap_or(String::from("")))?;
                        writeln!(file)?;
                        writeln!(file, "---")?;
                        writeln!(file, "Created at {}", note.created_at)?;
                        writeln!(file, "Updated at {}", note.updated_at)?;
                        writeln!(file, "Deleted at {}", note.deleted_at.unwrap_or(String::from("NA")))?;
                    }
                }
        } else {
            error!("No directory selected");
        }
        Ok(())
    }

    fn sanitize(&self, s: &str) -> String {
        let mut x = s.replace("/", "_")
            .replace(" ", "_");
        
        x = x.chars()
            .filter(|c| c.is_ascii())
            .collect();

        if x.len() > 100 {
            x.truncate(100);
        }
        x
    }
}
