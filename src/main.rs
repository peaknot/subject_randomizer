use clap::Parser;

use crate::{errors::AppError, handles::init_db, structs::{AppState, Cli}};

slint::include_modules!();

mod errors;
mod handles;
mod structs;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
    if let Err(err) = app_window() {
        eprintln!("{err}");
        std::process::exit(1);
    }
    
}

fn run() -> Result<(), AppError> {
    
    let conn = init_db("./pharma_subs.db")?;

    let app_state = AppState { db: conn};

    let args = Cli::parse();

    args.command.execute(app_state.db)
    
}

fn app_window() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    main_window.run()
}
