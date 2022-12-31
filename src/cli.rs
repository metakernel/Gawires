pub mod commands;
pub mod handling;

pub use clap::{Parser};
use crate::cli::commands::Commands;

    #[derive(Debug, PartialEq, Parser)]
    #[command(name = "gawires", version,  about = "Please use -h or --help to know more about gawires subcommands")]
    pub struct Cli {
        #[command(subcommand)]
        pub commands: Option<Commands>,
    }