use paw::Args;
pub use clap::Parser;

use gawires::cli::Cli;
use gawires::cli::handling::handle_cmds;

/// The main entry point for Gawires.
#[paw::main]
fn main(_args: Args) {
    let cli = Cli::parse();
    //println!("{:#?}", opt);  

    handle_cmds(cli.commands.unwrap());
}
