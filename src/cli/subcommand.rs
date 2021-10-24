use clap::{Parser};

/// Enum of all possible subcommands
#[derive(Debug, PartialEq,Parser)]
    pub enum Subcommand{
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

    ///track new assets or changes, add tags and other operations.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Add{
        /// Stage assets changes in a given path
            pub path: Option<std::path::PathBuf>,
            /// Stage all changes that are not ignored
            #[clap(short, long)]
            pub all: bool,
            /// Stop tracking the assets from a given path
            #[clap(name = "Path",short = 'i', long = "--ignore")]
            pub ignore_path: Option<std::path::PathBuf>,

            /// Add a tag to the assets with the given name
            #[clap(name = "Tag name",short = 't', long = "--tag")]
            pub tag_name: Option<String>,
    }

    /// Checkout assets in local workspace. When assets are checkout, they are locked by default when in centralized mode.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Checkout{
        /// Specific assets or complete directory structure can be checkout
        pub path: Option<std::path::PathBuf>,
        /// Option use to specify if a lock should be put on the assets, this will tell central to lock them. (On by default, WARNING: if centralized you should use "gawires checkout --clone" if you dont want any conflict issues)
        #[clap(name = "Lock",short = 'l', long = "--lock")]
        pub lock_assets: Option<bool>,
        /// Can be use to specify that the checkout must clone the assets instead of synchronizing them.(Will need to initiate a push request each time you try to synchronize)
        pub clone: Option<bool>,

    }
    /// Release checkout assets from local workspace.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Release{
    }
    /// Initialize a new project workspace
    #[derive(Debug, PartialEq, Parser)]
    pub struct Init{
    }

    /// While used in a distributed workspace this will fetch the remote, while in centralized workspace this will synchronize local workspace state with remote.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Sync{
    }

    /// Log and fetch information about local workspace
    #[derive(Debug, PartialEq, Parser)]
    pub struct Status{
    }

    /// Create and manage channels, channels are used to have specific assets versions or representation of a same project.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Channel{
    }

    /// Used in a distributed workspace to push local changes to remote.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Push{
    }

    /// Used in a distributed workspace to pull localy changes from remote.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Pull{
    }

    /// Commands to manage workspace
    #[derive(Debug, PartialEq, Parser)]
    pub struct Workspace{
    }

    /// Preview and compare different types of asset
    #[derive(Debug, PartialEq, Parser)]
    pub struct Preview{
    }

    /// Reset workspace assets to an earlier state
    #[derive(Debug, PartialEq, Parser)]
    pub struct Reset{
    }


    /// Unstage new assets or changes
    #[derive(Debug, PartialEq, Parser)]
    pub struct Remove{
    }

    /// Operations on branches
    #[derive(Debug, PartialEq, Parser)]
    pub struct Branch{
    }

    /// Add or remove tags from assets or project version
    #[derive(Debug, PartialEq, Parser)]
    pub struct Tag{
    }

    /// Create or configure wires.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Wire{
    }

    /// Connect and sync a Centralized Workspace.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Connect{
    }

    /// Filter operations
    #[derive(Debug, PartialEq, Parser)]
    pub struct Filter{
    }


    /// Rebase a branch or a centralized workspace
    #[derive(Debug, PartialEq, Parser)]
    pub struct Rebase{
    }

    /// Merge changes between 2 branches or centralized workspaces
    #[derive(Debug, PartialEq, Parser)]
    pub struct Merge{
    }

    /// Cleanup tools for local workspace
    #[derive(Debug, PartialEq, Parser)]
    pub struct Clean{
    }

    /// Commands related to the creation or management of a Gawires Central Server.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Central{
    }

    /// Install a Gawire extensions.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Install{
    }

    /// Uninstall a Gawire extensions.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Uninstall{
    }

    /// Operations over local or remote asset's gist
    #[derive(Debug, PartialEq, Parser)]
    pub struct Gist{
    }
