use clap::Parser;

use crate::{errors::AppError, handles::init_db, structs::Cli};

slint::include_modules!();

mod errors;
mod handles;
mod structs;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    // for now utilize a pre-made database
    // extract contents from the db
    // and get a random subject/topic
    let conn = init_db("./pharma_subs.db")?;

    let args = Cli::parse();

    args.command.execute(conn)
}
