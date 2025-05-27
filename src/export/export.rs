use eframe::egui;
use rfd::FileDialog;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use std::error::Error;
use log::{info, error};
use crate::app::{App};
//use std::time::Duration;
//use std::thread;

impl App {
    pub fn export(&mut self, target: &str) -> Result<(), Box<dyn Error>> {
        if self.state_exporting {
            println!("call early exit");
            return Ok(()); // exporting in progress, only one can be run!
        }

        let (tx, rx) = std::sync::mpsc::channel::<f32>();
        self.export_rx = Some(rx);
        self.state_exporting = true;
        self.state_export_progress = Some(0.0);
        
        if let Some(path) = FileDialog::new()
            .pick_folder() {
                
                // create an empty dir 'exported'
                let full_path = Path::new(&path).join("exported");
                fs::create_dir_all(&full_path)?;
                println!("Dir created at: {:?}", full_path);
                if target == "md" {
                    let db = crate::db::database::Database::new(&self.db_path)?;
                    let notes = db.get_all_notes()?;
                    let total = notes.len().max(1); // prevent division by 0
                    
                    std::thread::spawn(move || {                    
                        for (i, note) in notes.into_iter().enumerate() {
                            let safe_name = sanitize(&note.name);
                            let file_path = full_path.join(format!("{safe_name}.md"));

                            let mut file = match File::create(&file_path) {
                                Ok(f) => f,
                                Err(e) => {
                                    eprintln!("Failed to create file {:?}: {}", file_path, e);
                                    return;
                                }
                            };
                            
                            if !write_line(&mut file, &format!("# {}", note.name)) {return; };
                            if !write_line(&mut file, "") {return; };
                            if !write_line(&mut file, &note.content.unwrap_or(String::from(""))) {return; };
                            if !write_line(&mut file, "") {return; };
                            if !write_line(&mut file, "---") {return; };
                            if !write_line(&mut file, &format!("Created at {}", note.created_at)) {return; };
                            if !write_line(&mut file, &format!("Updated at {}", note.updated_at)) {return; };
                            if !write_line(&mut file, &format!("Deleted at {}", note.deleted_at
                                                                     .unwrap_or("NA".to_string()))) {return; };
                            
                            // progress
                            //thread::sleep(Duration::from_millis(300));
                            tx.send((i+1) as f32 / total as f32).ok();
                        }
                    });
                }
        } else {
            error!("No directory selected");
        }
        Ok(())
    }
}
fn sanitize(s: &str) -> String {
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
fn write_line(file: &mut File, line: &str) -> bool {
    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Failed to write to file: {e}");
        return false;
    }
    true
}
