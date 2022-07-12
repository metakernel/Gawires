use clap::{Parser};

// This is a work in progress. Most of those commands serve just as placeholders for future development. Still researching the best way to manage projects.

/// Enum of all possible subcommands
#[derive(Debug, PartialEq,Parser)]
    pub enum Subcommand{
        Add(Add), // Add new a files or folder content to the staging area, can be local, remote or one already followed by the gist.
        Checkout(Checkout), // Checkout specific files
        Release(Release), // Release a workspace
        Init(Init), // Initialize a connection to a remote project(For a centralized repos) or create a new project locally (For a distributed repos).
        Sync(Sync), // Sync the workspace with the distant repository (Will Pull & Push for a distributed repos).
        Status(Status), // Display the status of the local workspace
        Channel(Channel), // Change or create content channel for the workspace (This enables to switch between different versions of the same project)
        Push(Push), // Push the local workspace changes to the distant repository
        Pull(Pull), // Pull the distant repository changes to the local workspace
        Workspace(Workspace), // Display the workspace information or create/change to another workspace.
        Preview(Preview), 
        Reset(Reset), // Reset the local workspace to a specific commit (If non use the latest), can use --preserve to keep the local changes or --overwrite to delete them.
        Remove(Remove), // Remove a file or folder from the local workspace
        Branch(Branch), // Create a new branch inside the workspace
        Tag(Tag), // Add a tag to files or folders in the workspace
        Wire(Wire), // Commands related to Wires creation and configuration. Wires are bound to assets and can be used to specify how those assets are handled.
        Connect(Connect), // Start a LiveWire connection beetween local and remote workspace.
        Filter(Filter), // Filter files or folders in the workspace or project and return the filtered list
        Rebase(Rebase), // Rebase a workspace branch on another branch
        Merge(Merge), // Merge a workspace branch on another branch
        Clean(Clean), // Clean the workspace
        Central(Central), 
        Install(Install),
        Uninstall(Uninstall),
        Gist(Gist), // Gist and per asset actions.
        
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
        pub lock_remote: Option<bool>,
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
