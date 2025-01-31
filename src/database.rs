use rusqlite::{Connection, Result};

pub fn create_data() -> Result<()> {
    let conn = Connection::open("projek1.sqlite")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        done BOOLEAN NOT NULL DEFAULT FALSE
    )",
        (),
    )?;

    Ok(())
}
