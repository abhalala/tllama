use rusqlite::{Connection, Result};

pub fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("tllama.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            level TEXT NOT NULL,
            message TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub fn log_status(conn: &Connection, level: &str, message: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO logs (timestamp, level, message) VALUES (?1, ?2, ?3)",
        &[&chrono::Utc::now().to_rfc3339(), level, message],
    )?;
    Ok(())
}
