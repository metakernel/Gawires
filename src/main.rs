#![feature(const_option)]

mod cli;
use paw::Args;
pub use clap::Parser;

use crate::cli::Opts;
use crate::cli::handling::handle_subcmd;

/// The main entry point for Gawires.
#[paw::main]
fn main(_args: Args) {
    let opt = Opts::parse();
    //println!("{:#?}", opt);

    handle_subcmd(opt.subcommand);
}
