use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use crate::{errors::AppError, structs::AppState};
use rusqlite::Connection;

pub fn init_db() -> Result<AppState, AppError> {

    let conn = Connection::open(db_path())?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;


    Ok(AppState { 
        pool: Arc::new(Mutex::new(conn)) 
    })
}

pub fn db_path() -> PathBuf {
    
    let local_path = PathBuf::from("pharma_subs.db");
    if local_path.exists() {
        return local_path;
    }

    let mut path = dirs::data_local_dir()
        .expect("could not find data dir");
    path.push("subject_randomizer");
    std::fs::create_dir_all(&path).ok();
    path.push("pharma_subs.db");
    path
}
