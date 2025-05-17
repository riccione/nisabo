use std::path::PathBuf;
use rusqlite::{Connection, Result};
use crate::app::App;
use crate::config::AppConfig;

pub fn create_db(app: &mut App, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // create a sqlite db
    if path.try_exists()? {
        return Err(format!("DB already exists at {:?}", path).into());
    }
    
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS archive (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            name            TEXT NOT NULL,
            content         TEXT,
            created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
        (),
    )?;
    
    conn.execute(
       "INSERT INTO archive (
            name, content, created_at, updated_at
        ) VALUES (?1, ?2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ",
        (
            "README",
            "# Welcome to nisabo",
        ),
    )?;

    let config = AppConfig {
        last_archive_path: Some(path.clone()),
    };
    config.save_config();

    app.archive_path = Some(path.clone());

    Ok(())
}
