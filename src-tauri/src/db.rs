use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("passwords.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            login_id TEXT NOT NULL,
            password TEXT NOT NULL,
            url TEXT
        )",
        [],
    )?;
    Ok(conn)
}
