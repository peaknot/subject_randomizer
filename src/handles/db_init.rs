use crate::errors::AppError;
use rusqlite::Connection;

pub fn init_db(path: &str) -> Result<Connection, AppError> {
    let conn = Connection::open(path)?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(conn)
}
