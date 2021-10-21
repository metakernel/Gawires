mod cli;

use structopt::StructOpt;

use crate::cli::Opt;

#[paw::main]
fn main(_args: paw::Args) {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
