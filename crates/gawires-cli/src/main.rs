pub mod cli;

use paw::Args;
pub use clap::Parser;

use cli::Cli;
use crate::cli::handling::handle_cmds;

/// The main entry point for Gawires.
#[paw::main]
fn main(_args: Args) {
    let cli = Cli::parse();
    //println!("{:#?}", opt);  

    handle_cmds(cli.commands.unwrap());
}
