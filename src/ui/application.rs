use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use rusqlite::Connection;
use crate::{handles::init_db, structs::AppState, ui::*};


#[component]
pub fn app() -> Element {
    use_context_provider(|| -> AppState {
        init_db("./pharma_subs.db").unwrap()
    });
    rsx! {
        Card { }
    }
}