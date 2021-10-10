pub mod command;

pub use structopt::StructOpt;
use crate::cli::command::*;

    #[derive(Debug, PartialEq, StructOpt)]
    #[structopt(name = "gawires", about = "Please use -h or --help to know more about gawires commands")]
    pub enum Opt{
        Add(Add),
        Checkout(Checkout),
        Release(Release),
        Init(Init),
        Sync(Sync),
        Status(Status),
        Channel(Channel),
        Push(Push),
        Pull(Pull),
        Workspace(Workspace),
        Preview(Preview),
        Reset(Reset),
        Remove(Remove),
        Branch(Branch),
        Tag(Tag),
        Wire(Wire),
        Connect(Connect),
        Filter(Filter),
        Rebase(Rebase),
        Merge(Merge),
        Clean(Clean),
        Central(Central),
        Install(Install),
        Uninstall(Uninstall),
        Gist(Gist),
        
    }