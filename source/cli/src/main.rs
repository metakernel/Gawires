mod cli;

use structopt::StructOpt;
use gawires_core;

use crate::cli::Opt;

#[paw::main]
fn main(_args: paw::Args) {
    gawires_core::test();
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
