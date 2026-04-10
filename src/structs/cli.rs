use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::{
    errors::AppError,
    handles::{rand, rand_module, rand_subject, rand_topic},
};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: HalaBira,
}

#[derive(Subcommand)]
pub enum HalaBira {
    #[command(alias = "yolo", visible_alias = "Exe")]
    Exe,
    #[command(visible_alias = "Mod")]
    Mod,
    #[command(visible_alias = "Sub")]
    Sub,
    #[command(visible_alias = "Tpc")]
    Tpc,
}

impl HalaBira {
    pub fn execute(self, conn: Connection) -> Result<(), AppError> {
        match self {
            HalaBira::Exe => println!("{}", rand(&conn)?),
            HalaBira::Mod => println!("{}", rand_module(&conn)?),
            HalaBira::Sub => println!("{}", rand_subject(&conn)?),
            HalaBira::Tpc => println!("{}", rand_topic(&conn)?),
        }

        Ok(())
    }
}
