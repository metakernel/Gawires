use structopt::StructOpt;

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Stage{
        /// Stage assets changes in a given path
            path: Option<std::path::PathBuf>,
            /// Stage all changes that are not ignored
            #[structopt(short, long)]
            all: bool,
            /// Stop tracking the assets from a given path
            #[structopt(name = "Path",short = "-i", long = "--ignore")]
            ignore_path: Option<std::path::PathBuf>,
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Checkout{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Init{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Clone{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Sync{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Status{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Channel{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Lock{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Unlock{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Push{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Pull{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Mode{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Preview{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Reset{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Remove{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Branch{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Tag{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Wire{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Connect{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Filter{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Rebase{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Clean{
    }
    
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Central{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Install{
    }

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Gist{
    }
