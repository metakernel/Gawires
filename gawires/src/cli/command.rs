use structopt::StructOpt;

    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Add{
        /// Stage assets changes in a given path
            path: Option<std::path::PathBuf>,
            /// Stage all changes that are not ignored
            #[structopt(short, long)]
            all: bool,
            /// Stop tracking the assets from a given path
            #[structopt(name = "Path",short = "-i", long = "--ignore")]
            ignore_path: Option<std::path::PathBuf>,
    }
