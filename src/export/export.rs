use eframe::egui;
use rfd::FileDialog;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use std::error::Error;
use log::{info, error};
use crate::app::{App};
use pulldown_cmark::{Parser, Options, html};

impl App {
    pub fn export(&mut self, target: &str) -> Result<(), Box<dyn Error>> {
        if self.state_exporting {
            println!("call early exit");
            return Ok(()); // exporting in progress, only one can be run!
        }
        
        if let Some(path) = FileDialog::new()
            .pick_folder() {
                let (tx, rx) = std::sync::mpsc::channel::<f32>();
                self.export_rx = Some(rx);
                self.state_exporting = true;
                self.state_export_progress = Some(0.0);
                
                // create an empty dir 'exported'
                let full_path = Path::new(&path).join("exported");
                fs::create_dir_all(&full_path)?;
                println!("Dir created at: {:?}", full_path);
                
                    let db = crate::db::database::Database::new(&self.db_path)?;
                    let notes = db.get_all_notes()?;
                    let total = notes.len().max(1); // prevent division by 0
                    let format = target.to_string(); // to fix borrow issue
                    
                    std::thread::spawn(move || {                    
                        for (i, note) in notes.into_iter().enumerate() {
                            let safe_name = sanitize(&note.name);
                            let file_path = full_path.join(format!("{safe_name}.{format}"));

                            let mut file = match File::create(&file_path) {
                                Ok(f) => f,
                                Err(e) => {
                                    eprintln!("Failed to create file {:?}: {}", file_path, e);
                                    return;
                                }
                            };
                           
                            let data = format!(r#"---
title: "{name}"

date: {created}

updated: {updated}

deleted: {deleted}

draft: false

---

{content}
"#,
                                name = note.name,
                                content = &note.content.unwrap_or(String::from("")),
                                created = note.created_at,
                                updated = note.updated_at,
                                deleted = note.deleted_at.unwrap_or("NA".to_string())
                            );
                            
                            let output = match format.as_str() {
                                "html" => md_to_html(&data),
                                _ => data,
                            };
                            match file.write_all(output.as_bytes()) {
                                Ok(_) => info!("Html file saved: {safe_name}"),
                                Err(e) => eprintln!("Failed to save {safe_name}: {e}"),
                            }
                            // progress
                            tx.send((i+1) as f32 / total as f32).ok();
                        }
                    });
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

fn md_to_html(md: &str) -> String {
    let options = Options::empty();

    let parser = Parser::new_ext(md, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
