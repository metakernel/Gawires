pub mod command;

pub use structopt::StructOpt;
use crate::cli::command::*;

    #[derive(Debug, PartialEq, StructOpt)]
    #[structopt(name = "gawires", about = "Please use -h or --help to know more about gawires commands")]
    pub enum Cli{
        Add(Add),
    }