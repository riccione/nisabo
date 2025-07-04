use rfd::FileDialog;
use std::fs;
use std::error::Error;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use crate::app::{App};

impl App {
    pub fn import(&mut self) -> Result<(), Box<dyn Error>> {
        if self.state_importing || self.state_exporting {
            println!("call early exit");
            return Ok(()); // importing in progress, only one can be run!
        }
        
        if let Some(path) = FileDialog::new().pick_folder() {
            let (tx, rx) = std::sync::mpsc::channel::<f32>();
            self.io_rx = Some(rx);
            self.state_importing = true;
            self.state_io_progress = Some(0.0);
            
            let import_done = self.import_done.clone();
                
            let entries: Vec<_> = fs::read_dir(path)?
                .filter_map(Result::ok)
                .filter(|e| {
                    let path = e.path();
                    path.is_file() && path.extension().map(|ext| ext == "md").unwrap_or(false)
                })
                .collect();
            
            let total = entries.len().max(1); // prevents division to 0
            let db_path = self.db_path.clone();

            std::thread::spawn(move || {                    
            for (i, entry) in entries.iter().enumerate() {
                    let path = entry.path();

                    if let Some(filename) = path.file_stem().and_then(|x| x.to_str()) {
                            if let Ok(content) = fs::read_to_string(&path) {
                                println!("{}: {}", filename, content);
                                let mut db = match crate::db::database::Database::new(&db_path) {
                                    Ok(db) => db,
                                    Err(e) => {
                                        eprintln!("Failed to connect to db: {}", e);
                                        return;
                                    }
                                };
                                
                                if let Err(e) = db.insert_note(&filename, &content) {
                                    eprintln!("Failed to insert note: {}", e);
                                }
                        }
                    }
                    // progress
                    tx.send((i+1) as f32 / total as f32).ok();
            }
            import_done.store(true, Ordering::Relaxed);
                });
        } else {
        eprintln!("No directory selected");
        }
        self.load_rows = false;
        Ok(())
    }
}
