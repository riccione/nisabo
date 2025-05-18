use std::path::{Path, PathBuf};
use rusqlite::{Connection, Result};

pub fn create_db(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

pub fn ls(path: &Path) -> Result<Vec<String>> {
    let conn = Connection::open(path)?;

    let mut x = conn.prepare("SELECT name FROM archive")?;
    let archive_iter = x.query_map([], |row| row.get(0))?;

    let mut names = Vec::new();
    for x in archive_iter {
        names.push(x?);
    }

    Ok(names)
}
