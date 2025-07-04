use rusqlite::{Connection, params, Transaction, Result};
use crate::db::models::{LinkType, Note, NoteIdName, NoteLink, NoteDiff,  NoteLinkIds};

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
            
            CREATE TABLE IF NOT EXISTS note_diff (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id         INTEGER NOT NULL,
                version         INTEGER,
                diff            TEXT,
                changed_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (note_id) REFERENCES note(id) ON DELETE CASCADE
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS note_fts USING fts5 (
                id UNINDEXED,
                name,
                content,
                content='note',
            );
            
            CREATE TRIGGER IF NOT EXISTS note_ai AFTER INSERT ON note BEGIN
                INSERT INTO note_fts(rowid, name, content)
                VALUES (new.id, new.name, new.content);
                -- INSERT INTO note_fts(id, name, content)
                -- VALUES (new.id, new.name, new.content);
            END;

            CREATE TRIGGER IF NOT EXISTS note_au AFTER UPDATE ON note BEGIN
                -- delete the old entry
                INSERT INTO note_fts(note_fts, id, name, content) 
                VALUES ('delete', old.id, old.name, old.content);
                -- DELETE FROM note_fts WHERE rowid = old.id;
                -- insert the new entry
                INSERT INTO note_fts(rowid, name, content) 
                VALUES (new.id, new.name, new.content);
                -- INSERT INTO note_fts(id, name, content) 
                -- VALUES (new.id, new.name, new.content);
            END;

            CREATE TRIGGER IF NOT EXISTS note_ad AFTER DELETE ON note BEGIN
                -- DELETE FROM note_fts WHERE rowid = old.id;
                INSERT INTO note_fts(note_fts, rowid, name, content) 
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
        self.with_transaction(|tx| {
            tx.execute("
            INSERT INTO note (
                name, content, created_at, updated_at, deleted_at
            ) VALUES (?1, ?2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, NULL)
            ",
            ("README", "# Welcome to nisabo"),
            )?;

            Ok(())
        })
    }
    
    pub fn insert_note(&mut self, name: &str, content: &str) -> Result<()> {
        self.with_transaction(|tx| {
            tx.execute("
            INSERT INTO note (
                name, content, created_at, updated_at, deleted_at
            ) VALUES (?1, ?2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, NULL)
            ",
            (name, content),
            )?;

            Ok(())
        })
    }

    pub fn get_notes(&self) -> Result<Vec<NoteIdName>, rusqlite::Error> {
        let mut x = self.conn
            .prepare("SELECT id, name FROM note WHERE deleted_at is NULL ORDER BY updated_at DESC")?;
        let note_iter = x.query_map([], |row| {
            Ok(NoteIdName {
                id: row.get(0)?,
                name: row.get(1)?,
                children: vec![],
                has_parent: false,
            })
        })?;

        let mut notes = std::collections::HashMap::new();
        let mut ordered_notes: Vec<i64> = Vec::new();

        for xs in note_iter {
            let note = xs?;
            let note_id = note.id; // to resolve moved value for ordered_notes
            notes.insert(note.id, note);
            ordered_notes.push(note_id);
        }

        let mut link_stmt = self.conn.prepare("
            SELECT source_note_id, target_note_id FROM note_link 
            WHERE link_type = 'parent'")?;

        let link_iter = link_stmt.query_map([], |row| {
            Ok(NoteLinkIds {
                source_note_id: row.get(0)?,
                target_note_id: row.get(1)?
            })
        })?;

        for xs in link_iter {
            let link =  xs?;
            let parent_id = link.source_note_id;
            let child_id = link.target_note_id;

            if let Some(mut child) = notes.remove(&child_id) {
                child.has_parent = true;
                if let Some(parent) = notes.get_mut(&parent_id) {
                    parent.children.push(child);
                }
            }
        }
        let top_level_notes: Vec<NoteIdName> = ordered_notes
            .into_iter()
            .filter_map(|id| notes.remove(&id))
            .collect();
        Ok(top_level_notes)
    }
    
    pub fn update_note_name(&mut self, id: i64, new_name: &str) -> Result<()> {
        self.with_transaction(|tx| {
            tx.execute(
                "UPDATE note SET name = ?1 WHERE id = ?2",
                (new_name, id),
            )?;
            Ok(())
        })
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
   
    pub fn delete_note_and_children_soft(&mut self, id: i64) -> Result<()> {
        self.delete_note_soft(id)?;
        self.delete_note_link_soft(id)?;

        Ok(())
    }

    fn delete_note_soft(&mut self, id: i64) -> Result<()> {
        self.with_transaction(|tx| {
            tx.execute(
                "UPDATE note SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?1",
                &[&id],
            )?;
            Ok(())
        })
    }
    
    fn delete_note_link_soft(&mut self, id: i64) -> Result<()> {
        self.with_transaction(|tx| {
            tx.execute(
                "UPDATE note SET deleted_at = CURRENT_TIMESTAMP 
                WHERE id IN (
                    SELECT target_note_id
                    FROM note_link
                    WHERE source_note_id = ?1 AND link_type = ?2
                )",
                params![id, LinkType::Parent.to_string()],
            )?;
            Ok(())
        })
    }
    
    pub fn delete_note_hard(&mut self, id: i64) -> Result<()> {
        self.with_transaction(|tx| {
            tx.execute(
                "DELETE FROM note WHERE id = ?1",
                &[&id],
            )?;
            Ok(())
        })
    }
    
    pub fn empty_trash(&mut self) -> Result<()> {
        self.with_transaction(|tx| {
            tx.execute(
                "DELETE FROM note WHERE deleted_at IS NOT NULL",
                [],
            )?;
            Ok(())
        })
        //let mut stmt = self.conn.prepare("DELETE FROM note WHERE deleted_at IS NOT NULL")?;
        //stmt.execute([])?;
        //Ok(())
    }
    
    pub fn restore_note(&mut self, id: i64) -> Result<()> {
        self.with_transaction(|tx| {
            tx.execute(
                "UPDATE note SET deleted_at = NULL WHERE id = ?1",
                &[&id],
            )?;
            Ok(())
        })
    }
    
    pub fn add_new_note(&mut self, name: &str) -> Result<i64> {
        self.with_transaction(|tx| {
            tx.execute(
                "INSERT INTO note (name, created_at, updated_at, deleted_at) 
                VALUES (?1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, NULL)",
                &[&name],
            )?;

            let id = tx.last_insert_rowid();
            Ok(id)
        })
    }
    
    pub fn add_note_link(&mut self, source_note_id: i64, target_note_id: i64, link_type: LinkType) -> Result<()> {
        self.with_transaction(|tx| {
            tx.execute(
                "INSERT INTO note_link (source_note_id, target_note_id, link_type) 
                VALUES (?1, ?2, ?3)",
                params![source_note_id, target_note_id, link_type.to_string()],
            )?;

            Ok(())
        })
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
    
    pub fn update_note_content(&mut self, id: i64, new_content: &str) -> Result<()> {
        self.with_transaction(|tx| {
            tx.execute(
                "UPDATE note SET content = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
                params![new_content, id],
            )?;
            Ok(())
        })
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

    pub fn search(&self, query: &str) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT n.id, n.name, n.content
            FROM note_fts fts 
            JOIN note n ON n.id = fts.rowid 
            WHERE note_fts MATCH ?1 
            AND n.deleted_at IS NULL 
            ORDER BY rank"
        )?; 
        
        let note_iter = stmt.query_map(params![query], |row| {
            Ok(Note {
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                created_at: String::new(),
                updated_at: String::new(),
                deleted_at: None,
            })
        })?;
        
        note_iter.collect()
    }

    // draft
    pub fn insert_note_diff(&mut self, note_id: i64, diff: &str) -> Result<()> {
        let mut version = match self.select_latest_note_diff_version(note_id) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("No version value: {}", e);
                // if there are no records => Err(QueryReturnedNoRows)
                // and we set version to 0
                // it is an initial state
                0
            }
        };
        version += 1;
        
        self.with_transaction(|tx| {
            tx.execute("
            INSERT INTO note_diff (
                note_id, version, diff, changed_at
            ) VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)
            ",
            (note_id, version, diff),
            )?;

            Ok(())
        })
    }
    
    pub fn select_note_diff_ls(&mut self, note_id: i64) -> Result<Vec<NoteDiff>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, version, changed_at 
            FROM note_diff 
            WHERE note_id = ?1 
            ORDER BY version"
        )?;
        let note_diff_iter = stmt.query_map(params![note_id], |row| {
            Ok(NoteDiff {
                id: row.get(0)?,
                version: row.get(1)?,
                diff: String::new(), // dummy value for the list
                changed_at: row.get(2)?,
            })
        })?;
        note_diff_iter.collect()
    }

    pub fn select_note_diff(&mut self, id: i64) -> Result<NoteDiff> {
        self.conn.query_row(
            "SELECT id, version, diff, changed_at FROM note_diff WHERE id = ?1",
            [&id],
            |row| {
                Ok(NoteDiff {
                    id: row.get(0)?,
                    version: row.get(1)?,
                    diff: row.get(2)?,
                    changed_at: row.get(3)?,
                })
            },
        )
    }
    
    fn select_latest_note_diff_version(&mut self, note_id: i64) -> Result<i64> {
        self.conn.query_row(
            "SELECT version 
            FROM note_diff WHERE note_id = ?1 
            ORDER BY version DESC 
            LIMIT 1",
            [&note_id],
            |row| row.get(0),
        )
    }
}
