use rusqlite::{Connection, params, Result};
use chrono::Utc;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database {conn})
    }

    pub fn init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS note (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                name            TEXT NOT NULL,
                content         TEXT,
                created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                deleted_at      DATETIME
                )",
            (),
        )?;
        
        self.conn.execute(
           "INSERT INTO note (
                name, content, created_at, updated_at, deleted_at
            ) VALUES (?1, ?2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, NULL)
            ",
            (
                "README",
                "# Welcome to nisabo",
            ),
        )?;
        Ok(())
    }

    pub fn get_notes(&self) -> Result<Vec<(i32, String)>> {
        let mut x = self.conn.prepare("SELECT id, name FROM note WHERE deleted_at is NULL")?;
        let rows = x.query_map([], |row| {
            Ok((
                row.get(0)?, 
                row.get(1)?
            ))
        })?;

        let names = rows.collect::<Result<Vec<_>,_>>()?;
        Ok(names)
    }

    pub fn update_note_name(&self, id: i32, new_name: &str) -> Result<usize> {
        self.conn.execute(
            "UPDATE note SET name = ?1 WHERE id = ?2",
            (new_name, id),
        )
    }
    
    pub fn get_trash(&self) -> Result<Vec<(i32, String)>> {
        let mut x = self.conn.prepare("SELECT id, name FROM note WHERE deleted_at IS NOT NULL")?;
        let rows = x.query_map([], |row| {
            Ok((
                row.get(0)?, 
                row.get(1)?
            ))
        })?;

        let xz = rows.collect::<Result<Vec<_>,_>>()?;
        Ok(xz)
    }
    
    pub fn delete_note_soft(&self, id: i32) -> Result<usize> {
        let deleted_at = Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        self.conn.execute(
            "UPDATE note SET deleted_at = ?1 WHERE id = ?2",
            (deleted_at, id),
        )
    }
    
    pub fn delete_note_hard(&self, id: i32) -> Result<usize> {
        self.conn.execute(
            "DELETE FROM note WHERE id = ?1",
            &[&id],
        )
    }
    
    pub fn restore_note(&self, id: i32) -> Result<usize> {
        self.conn.execute(
            "UPDATE note SET deleted_at = NULL WHERE id = ?1",
            &[&id],
        )
    }
    
    pub fn add_new_note(&self, name: &str) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO note (
                name, created_at, updated_at, deleted_at
            ) VALUES (?1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, NULL)",
            &[&name],
        )
    }
    
    pub fn get_note(&self, id: i32) -> Result<(i32, String, String)> {
        self.conn.query_row(
            "SELECT id, name, content FROM note WHERE id = ?1",
            &[&id],
            |row| {
                let id: i32 = row.get(0)?;
                let name: String = row.get(1)?;
                let content: String = row.get(2)?;
                Ok((id, name, content))
            }
        )
    }
    
    pub fn update_note_content(&self, id: i32, new_content: &str) -> Result<usize> {
        let updated_at = Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        self.conn.execute(
            "UPDATE note SET content = ?1, updated_at = ?2 WHERE id = ?3",
            params![new_content, updated_at, id],
        )
    }
}
