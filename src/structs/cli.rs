use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::{errors::AppError, handles::randomize::rand, structs::module::Randomized};

#[derive(Parser)]
pub struct Cli {   
    #[command(subcommand)]
    
    pub command: HalaBira,
}

#[derive(Subcommand)]
pub enum HalaBira {
    #[command(alias = "yolo")]
    Exe,
}

impl HalaBira {
    pub fn execute(self, conn: Connection) -> Result<(), AppError> {
        let res = rand(conn)?;
        println!("{res}");
        Ok(())
    }
}