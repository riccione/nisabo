use std::path::{Path, PathBuf};
use rusqlite::{Connection, Result};
use chrono::{NaiveDateTime};
use log::{info, error};
use crate::app::App;

pub fn create_db(path: &PathBuf) -> Result<()> {
    // create a sqlite db
    println!("{:?}", path);
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

    Ok(())
}
