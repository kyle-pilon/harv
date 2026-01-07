use rusqlite::{Connection, Result};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create or open a database at the given path
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    /// Initialize the database schema
    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS thoughts (
                id TEXT PRIMARY KEY,
                raw_input TEXT NOT NULL,
                captured_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                current_state TEXT NOT NULL
            )",
            [],
        )?;

        // Create indexes for common queries
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_thoughts_state
            ON thoughts(current_state)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_thoughts_captured_at
            ON thoughts(captured_at)",
            [],
        )?;

        Ok(())
    }
}