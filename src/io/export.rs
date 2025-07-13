use rfd::FileDialog;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use std::error::Error;
use log::{info, error};
use crate::app::{App, IoOperation, ProgressState};
use pulldown_cmark::{Parser, Options, html};
use crate::db::models::Note;

impl App {
    pub fn export(&mut self, target: &str) -> Result<(), Box<dyn Error>> {
        if self.state_exporting {
            println!("call early exit");
            return Ok(()); // exporting in progress, only one can be run!
        }
        
        if let Some(path) = FileDialog::new()
            .pick_folder() {
                let (tx, rx) = std::sync::mpsc::channel::<f32>();
                self.io_rx = Some(rx);
                self.state_exporting = true;
                self.state_io_progress = Some(0.0);
                
                // create an empty dir 'exported'
                let full_path = Path::new(&path).join("exported");
                fs::create_dir_all(&full_path)?;
                println!("Dir created at: {:?}", full_path);

                let db = crate::db::database::Database::new(&self.db_path)?;
                let notes = db.get_all_notes()?;
                let total = notes.len().max(1); // prevent division by 0
                let format = target.to_string(); // to fix borrow issue

                self.io_operation = Some(IoOperation::Export);
                let handle = std::thread::spawn(move || -> Result<usize, String> {                    
                    let mut actual = 0;

                    for (i, note) in notes.into_iter().enumerate() {
                        let safe_name = sanitize(&note.name);
                        // solving same name issue by adding _{i}
                        let file_path = full_path.join(format!("{safe_name}_{i}.{format}"));

                        let mut file = match File::create(&file_path) {
                            Ok(f) => f,
                            Err(e) => {
                                //eprintln!("Failed to create file {:?}: {}", file_path, e);
                                let m = format!("Failed to create a file {:?}: {}", file_path, e);
                                return Err(m);
                                // return;
                            }
                        };
                        
                        let data = format_note_as_md(&note);
                            let output = match format.as_str() {
                                "html" => md_to_html(&data),
                                _ => data,
                            };
                            match file.write_all(output.as_bytes()) {
                                Ok(_) => {
                                    actual += 1;
                                    info!("Html file saved: {safe_name}");
                                }
                                Err(e) => {
                                    let m = format!("Failed to save {safe_name}: {e}");
                                    return Err(m);
                                    // eprintln!("Failed to save {safe_name}: {e}"),
                                }
                            }
                            // progress
                            tx.send((i+1) as f32 / total as f32).ok();
                        }
                    Ok(actual)
                });
            let mut has_error = false;
            let mut result = vec![];
            match handle.join() {
                Ok(Ok(actual)) => { // Success
                    let m = format!("Successfully exported from {}: {}", total, actual);
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
                    self.state_progress = ProgressState::Failed("Export failed".to_string());
                } else {
                    self.state_progress = ProgressState::Completed("Export completed".to_string());
                }
            }
        } else {
            self.status_error = "No directory selected".to_string();
        }
        Ok(())
    }
}

fn format_note_as_md(note: &Note) -> String {
format!(r#"---
title: "{name}"

date: {created}

updated: {updated}

deleted: {deleted}

draft: false

---

{content}
"#,
    name = note.name,
    content = &note.content.clone().unwrap_or(String::from("")),
    created = note.created_at,
    updated = note.updated_at,
    deleted = note.deleted_at.clone().unwrap_or("NA".to_string())
    )
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
