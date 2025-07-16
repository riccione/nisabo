use rfd::FileDialog;
use std::fs;
use std::error::Error;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use crate::app::{App, ProgressState, IoOperation};

impl App {
    pub fn import(&mut self) -> Result<(), Box<dyn Error>> {

        if let ProgressState::InProgress(_) = self.state_progress {
            // call early exit
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
            self.io_operation = Some(IoOperation::Import);

            let handle = std::thread::spawn(move || -> Result<(), String> {                    
                for (i, entry) in entries.iter().enumerate() {
                        let path = entry.path();

                        if let Some(filename) = path.file_stem().and_then(|x| x.to_str()) {
                                if let Ok(content) = fs::read_to_string(&path) {
                                    let mut db = match crate::db::database::Database::new(&db_path) {
                                        Ok(db) => db,
                                        Err(e) => {
                                            let m = format!("Failed to connect to db: {}", e);
                                            // eprintln!("Failed to connect to db: {}", e);
                                            // return;
                                            return Err(m);
                                        }
                                    };
                                    
                                    if let Err(e) = db.insert_note(&filename, &content) {
                                        let m = format!("Failed to insert note {}: {}", filename, e);
                                        // eprintln!("Failed to insert note: {}", e);
                                        //return;
                                        return Err(m);
                                    }
                            }
                        }
                        // progress
                        tx.send((i+1) as f32 / total as f32).ok();
                }
                import_done.store(true, Ordering::Relaxed);
                Ok(())
            });
            let mut has_error = false;
            let mut result = vec![];
            match handle.join() {
                Ok(Ok(())) => { // Success
                    let m = format!("Successfully imported: {}", total);
                    result.push(m);
                },
                Ok(Err(e)) => {
                    has_error = true;
                    result.push(e);
                }
                Err(e) => {
                    has_error = true;
                    result.push(format!("Thread panicked: {:?}", e));
                }
            }
            if !result.is_empty() {
                self.io_status = result.join("\n");

                if has_error {
                    self.state_progress = ProgressState::Failed("Import failed".to_string());
                } else {
                    self.state_progress = ProgressState::Completed("Import completed".to_string());
                }
            }
        } else {
            self.status_error = "No directory selected".to_string();
            // eprintln!("No directory selected");
        }
        self.load_rows = false;
        Ok(())
    }
}
