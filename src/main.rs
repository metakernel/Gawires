mod cli;

use structopt::StructOpt;
use paw::Args;

use crate::cli::Opt;

/// The main entry point for Gawires.
#[paw::main]
fn main(_args: Args) {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
