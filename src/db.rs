use rusqlite::{Connection, Result};

fn init_db() -> Result<Connection> {
    let conn = Connection::open("/usr/local/rustbrew/rustbrew.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS packages (
            name TEXT PRIMARY KEY,
            version TEXT,
            installed_at INTEGER
        )",
        [],
    )?;
    Ok(conn)
}

fn update_db(formula: &Formula) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT OR REPLACE INTO packages (name, version, installed_at) VALUES (?1, ?2, ?3)",
        (&formula.name, &formula.version, chrono::Utc::now().timestamp()),
    )?;
    Ok(())
}
