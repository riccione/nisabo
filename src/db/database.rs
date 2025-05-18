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
            "CREATE TABLE IF NOT EXISTS note (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                name            TEXT NOT NULL,
                content         TEXT,
                created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
            (),
        )?;
        
        self.conn.execute(
           "INSERT INTO note (
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

    pub fn get_notes(&self) -> Result<Vec<(i32, String)>> {
        let mut x = self.conn.prepare("SELECT id, name FROM note")?;
        let rows = x.query_map([], |row| {
            Ok((
                row.get(0)?, 
                row.get(1)?
            ))
        })?;

        let names = rows.collect::<Result<Vec<_>,_>>()?;
        Ok(names)
    }
}
