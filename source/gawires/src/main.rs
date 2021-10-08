mod cli;

use structopt::StructOpt;
use gwcore;

use crate::cli::Opt;

#[paw::main]
fn main(_args: paw::Args) {
    gwcore::test();
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
