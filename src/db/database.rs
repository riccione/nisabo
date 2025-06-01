use rusqlite::{Connection, params, Transaction, Result};
use crate::db::models::{LinkType, Note, NoteIdName, NoteLink};
use chrono::Utc;

pub struct Database {
    conn: Connection,
}

impl Database {

    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database {conn})
    }

    fn with_transaction<F, T>(&mut self, f: F) -> Result<T> 
        where 
            F: FnOnce(&Transaction) -> Result<T>, {
                let tx = self.conn.transaction()?;
                let result = f(&tx)?;
                tx.commit()?;
                Ok(result)
    }

    pub fn configure_db(&self) -> Result<()> {
        self.conn.execute_batch("
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA cache_size = 10000;
            PRAGMA temp_store = MEMORY;
            PRAGMA case_sensitive_like = OFF;
            PRAGMA foreign_keys = ON;
        ")?;

        Ok(())
    }

    pub fn init_tables(&mut self) -> Result<()> {
        let r = self.with_transaction(|tx| {
            tx.execute_batch("
            CREATE TABLE IF NOT EXISTS note (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                name            TEXT NOT NULL,
                content         TEXT,
                created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                deleted_at      DATETIME
            );

            CREATE TABLE IF NOT EXISTS note_link (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                source_note_id  INTEGER NOT NULL,
                target_note_id  INTEGER NOT NULL,
                link_type       TEXT NOT NULL CHECK(link_type IN ('related', 'parent')),
                created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                deleted_at      DATETIME,
                FOREIGN KEY (source_note_id) REFERENCES note(id) ON DELETE CASCADE,
                FOREIGN KEY (target_note_id) REFERENCES note(id) ON DELETE CASCADE,
                UNIQUE (source_note_id, target_note_id, link_type) -- prevents duplications
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS note_fts USING fts5 (
                id UNINDEXED,
                name,
                content,
                content='note',
            );
            
            CREATE TRIGGER IF NOT EXISTS note_ai AFTER INSERT ON note BEGIN
                INSERT INTO note_fts(id, name, content)
                VALUES (new.id, new.name, new.content);
            END;

            CREATE TRIGGER IF NOT EXISTS note_au AFTER UPDATE ON note BEGIN
                -- delete the old entry
                INSERT INTO note_fts(note_fts, id, name, content) 
                VALUES ('delete', old.id, old.name, old.content);
                -- insert the new entry
                INSERT INTO note_fts(id, name, content) 
                VALUES (new.id, new.name, new.content);
            END;

            CREATE TRIGGER IF NOT EXISTS note_ad AFTER DELETE ON note BEGIN
                INSERT INTO note_fts(note_fts, id, name, content) 
                VALUES ('delete', old.id, old.name, old.content);
            END;
            ")?;


            Ok(())
        });
        eprintln!("Create tables: {:?}", r);
        
        // Insert dummy data
        let _ = self.insert_dummy_note();

        Ok(())
    }

    fn insert_dummy_note(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
           "INSERT INTO note (
                name, content, created_at, updated_at, deleted_at
            ) VALUES (?1, ?2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, NULL)
            ",
            ("README", "# Welcome to nisabo"),
        )?;

        tx.commit()
    }

    pub fn get_notes(&self) -> Result<Vec<NoteIdName>, rusqlite::Error> {
        let mut x = self.conn
            .prepare("SELECT id, name FROM note WHERE deleted_at is NULL ORDER BY updated_at DESC")?;
        let rows = x.query_map([], |row| {
            Ok(NoteIdName {
                id: row.get(0)?, 
                name: row.get(1)?,
            })
        })?;

        rows.collect()
    }
    
    pub fn update_note_name(&self, id: i64, new_name: &str) -> Result<usize> {
        self.conn.execute(
            "UPDATE note SET name = ?1 WHERE id = ?2",
            (new_name, id),
        )
    }
    
    pub fn get_trash(&self) -> Result<Vec<(i64, String)>> {
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
    
    pub fn delete_note_soft(&self, id: i64) -> Result<usize> {
        let deleted_at = Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        self.conn.execute(
            "UPDATE note SET deleted_at = ?1 WHERE id = ?2",
            (deleted_at, id),
        )
    }
    
    pub fn delete_note_hard(&self, id: i64) -> Result<usize> {
        self.conn.execute(
            "DELETE FROM note WHERE id = ?1",
            &[&id],
        )
    }
    
    pub fn restore_note(&self, id: i64) -> Result<usize> {
        self.conn.execute(
            "UPDATE note SET deleted_at = NULL WHERE id = ?1",
            &[&id],
        )
    }
    
    pub fn add_new_note(&self, name: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO note (
                name, created_at, updated_at, deleted_at
            ) VALUES (?1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, NULL)",
            &[&name],
        );
        
        let id = self.conn.last_insert_rowid();

        Ok(id)
    }
    
    pub fn add_note_link(&self, source_note_id: i64, target_note_id: i64, link_type: LinkType) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO note_link (source_note_id, target_note_id, link_type) 
            VALUES (?1, ?2, ?3)",
            params![source_note_id, target_note_id, link_type.to_string()],
        )
    }
    
    pub fn get_note(&self, id: i64) -> Result<Note> {
        self.conn.query_row(
            "SELECT * FROM note WHERE id = ?1",
            [&id],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                    deleted_at: row.get(5)?,
                })
            },
        )
    }
    
    pub fn update_note_content(&mut self, id: i64, new_content: &str) -> Result<usize> {
        let updated_at = Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let tx = self.conn.transaction()?;
        let ru = tx.execute(
            "UPDATE note SET content = ?1, updated_at = ?2 WHERE id = ?3",
            params![new_content, updated_at, id],
        )?;
        tx.commit()?;
        Ok(ru)
    }

    pub fn get_all_notes(&self) -> Result<Vec<Note>> {
        let mut x = self.conn.prepare("SELECT * FROM note WHERE deleted_at IS NULL")?;
        
        let iter = x.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                deleted_at: row.get(5)?,
            })
        })?;

        let mut notes = Vec::new();
        for y in iter {
            notes.push(y?);
        }

        Ok(notes)
    }
}
