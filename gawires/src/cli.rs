pub mod command;

pub use structopt::StructOpt;
use crate::cli::command::*;

    #[derive(Debug, PartialEq, StructOpt)]
    #[structopt(name = "gawires", about = "Please use -h or --help to know more about gawires commands")]
    pub enum Opt{
        Stage(Stage),
        Checkout(Checkout),
        Init(Init),
        Clone(Clone),
        Sync(Sync),
        Status(Status),
        Channel(Channel),
        Lock(Lock),
        Unlock(Unlock),
        Push(Push),
        Pull(Pull),
        Mode(Mode),
        Preview(Preview),
        Reset(Reset),
        Remove(Remove),
        Branch(Branch),
        Tag(Tag),
        Wire(Wire),
        Connect(Connect),
        Filter(Filter),
        Rebase(Rebase),
        Central(Central),
        Install(Install),
        Gist(Gist),
        
    }