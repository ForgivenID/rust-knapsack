use rusqlite::{Connection, Result, params};

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("knapsack.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS videos (
                hash TEXT PRIMARY KEY,
                metadata BLOB NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                hash TEXT PRIMARY KEY,
                video_hash TEXT NOT NULL,
                data BLOB NOT NULL,
                FOREIGN KEY(video_hash) REFERENCES videos(hash)
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn store_video(&self, hash: &str, metadata: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO videos (hash, metadata) VALUES (?1, ?2)",
            params![hash, metadata],
        )?;
        Ok(())
    }

    pub fn store_chunk(&self, chunk_hash: &str, video_hash: &str, data: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO chunks (hash, video_hash, data) VALUES (?1, ?2, ?3)",
            params![chunk_hash, video_hash, data],
        )?;
        Ok(())
    }
}