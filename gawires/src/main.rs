mod cli;

use structopt::StructOpt;

use crate::cli::Cli;

#[paw::main]
fn main(_args: paw::Args) {
    let opt = Cli::from_args();
    println!("{:#?}", opt);
}
