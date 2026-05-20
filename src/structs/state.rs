use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Mutex<rusqlite::Connection>>
}