#![warn(missing_docs)]
mod cli;

use structopt::StructOpt;
use paw::Args;

use crate::cli::Opt;

#[paw::main]
fn main(_args: Args) {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
