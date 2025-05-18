use rusqlite::{Connection, Result};

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
            "CREATE TABLE IF NOT EXISTS archive (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                name            TEXT NOT NULL,
                content         TEXT,
                created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
            (),
        )?;
        
        self.conn.execute(
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
}
