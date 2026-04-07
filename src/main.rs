use clap::Parser;

use crate::{handles::{init_db, randomize::rand}, structs::{Cli, cli::HalaBira}};

slint::include_modules!();

mod errors;
mod structs;
mod handles;
fn main() {
    // for now utilize a pre-made database
    // extract contents from the db
    // and get a random subject/topic
    let conn = init_db("./pharma_subs.db").unwrap();

    let args = Cli::parse();

    match args.command {
        HalaBira::Exe => { args.command.execute(conn).unwrap()}
    }
    
//    let res = rand(conn).unwrap();
//    println!("{res}");
    
}


