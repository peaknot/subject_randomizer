use std::sync::{Arc, Mutex};

use crate::{errors::AppError, structs::AppState};
use rusqlite::Connection;

pub fn init_db(path: &str) -> Result<AppState, AppError> {
    let conn = Connection::open(path)?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;


    Ok(AppState { 
        pool: Arc::new(Mutex::new(conn)) 
    })
}
