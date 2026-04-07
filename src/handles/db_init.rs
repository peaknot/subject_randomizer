use rusqlite::Connection;
use crate::errors::AppError;


pub fn init_db(path: &str) -> Result<Connection, AppError>{

    let conn = Connection::open(path)?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    
    Ok(conn)
}