use clap::Parser;
use crate::{errors::AppError, handles::init_db, structs::{AppState, Cli}, };
use dioxus::{prelude::*};
use crate::ui::*;

mod errors;
mod handles;
mod structs;
mod ui;

fn main() {
    dioxus::launch(app);
}




