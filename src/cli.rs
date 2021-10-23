pub mod subcommand;
pub mod handling;

pub use clap::{AppSettings,Parser};
use crate::cli::subcommand::Subcommand;

    #[derive(Debug, PartialEq, Parser)]
    #[clap(name = "gawires", about = "Please use -h or --help to know more about gawires subcommands")]
    pub struct Opts {
        #[clap(subcommand)]
        pub subcommand: Subcommand,
    }