use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::fs;
use crate::{errors::AppError, structs::AppState};
use rusqlite::Connection;

// Embed the database bytes into the binary at compile-time
const DB_BYTES: &[u8] = include_bytes!("../../pharma_subs.db");

pub fn init_db() -> Result<AppState, AppError> {
    let path = db_path();
    
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(AppError::Io)?;
        }
        fs::write(&path, DB_BYTES).map_err(AppError::Io)?;
    }

    let conn = Connection::open(&path).map_err(AppError::Db)?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(AppState { 
        pool: Arc::new(Mutex::new(conn)) 
    })
}

pub fn db_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .expect("Could not find system data directory");
    
    path.push("subject_randomizer");
    path.push("pharma_subs.db");
    path
}
