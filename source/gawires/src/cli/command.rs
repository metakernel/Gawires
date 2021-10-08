use structopt::StructOpt;

    ///track new assets or changes, add tags and other operations.
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

    /// Checkout assets in local workspace.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Checkout{
    }
    /// Release checkout assets from local workspace.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Release{
    }
    /// Initialize a new project workspace
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Init{
    }

    /// While used in a distributed workspace this will fetch the remote, while in centralized workspace this will synchronize local workspace state with remote.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Sync{
    }

    /// Log and fetch information about local workspace
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Status{
    }

    /// Create and manage channels, channels are used to have specific assets versions or representation of a same project.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Channel{
    }

    /// Used in a distributed workspace to push local changes to remote.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Push{
    }

    /// Used in a distributed workspace to pull localy changes from remote.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Pull{
    }

    /// Commands to manage workspace
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Workspace{
    }

    /// Preview and compare different types of asset
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Preview{
    }

    /// Reset workspace assets to an earlier state
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Reset{
    }


    /// Unstage new assets or changes
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Remove{
    }

    /// Operations on branches
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Branch{
    }

    /// Add or remove tags from assets or project version
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Tag{
    }

    /// Create or configure wires.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Wire{
    }

    /// Connect and sync a Centralized Workspace.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Connect{
    }

    /// Filter operations
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Filter{
    }


    /// Rebase a branch or a centralized workspace
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Rebase{
    }

    /// Merge changes between 2 branches or centralized workspaces
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Merge{
    }

    /// Cleanup tools for local workspace
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Clean{
    }

    /// Commands related to the creation or management of a Gawires Central Server.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Central{
    }

    /// Install a Gawire extensions.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Install{
    }

    /// Uninstall a Gawire extensions.
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Uninstall{
    }

    /// Operations over local or remote asset's gist
    #[derive(Debug, PartialEq, StructOpt)]
    pub struct Gist{
    }
