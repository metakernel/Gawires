use clap::{Parser,Subcommand};

// This is a work in progress. Most of those commands serve just as placeholders for future development. Still researching the best way to manage projects.

/// Enum of all possible subcommands
#[derive(Debug, PartialEq,Subcommand)]
    pub enum Commands{
        Add(Add), // Add new assets or changes to the workspace, add tags and other operations.
        Checkout(Checkout), // Checkout contents from a remote workspace.
        Release(Release), // Release a workspace
        Init(Init), // Initialize a connection to a remote project or workspace(For a centralized workflow).
        Sync(Sync), // Sync the workspace with the distant repository (Will Pull & Push for a distributed workflow).
        Clone(Clone), // Clone a remote workspace locally, it will clone a complete copy of it.
        Status(Status), // Display the status of the local workspace
        Layout(Layout), // Change or create content layout for the workspace (This enables to switch between different layout for the same workspace)
        Push(Push), // Push the local workspace changes to the distant repository
        Pull(Pull), // Pull the distant repository changes to the local workspace
        Workspace(Workspace), // Display the workspace information or create/change to another workspace.
        Preview(Preview), // Preview and compare different commit or assets
        Reset(Reset), // Reset the local workspace to a specific commit (If non use the latest), can use --preserve to keep the local changes or --overwrite to delete them.
        Remove(Remove), // Remove a file or folder from the local workspace
        Branch(Branch), // Create a new branch inside the workspace
        Wire(Wire), // Commands related to Wires creation and configuration. Wires are bound to assets and can be used to specify how those assets are handled.
        Connect(Connect), // Start a LiveWire connection beetween local and remote workspace.
        Filter(Filter), // Filter files or folders in the workspace or project and return the filtered list
        Rebase(Rebase), // Rebase a workspace branch on another branch
        Merge(Merge), // Merge a workspace branch on another branch
        Clean(Clean), // Clean the workspace
        Project(Project), 
        Install(Install),
        Uninstall(Uninstall),
        Asset(Asset),
        
    }

    ///track new assets or changes, add tags and other operations.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Add{
        /// Stage assets changes in a given path
            pub path: Option<std::path::PathBuf>,
            /// Stage all changes in workspace that are not ignored
            #[arg(short = 'a', long = "all")]
            pub all: bool,
            /// Stop tracking the assets from a given path
            #[arg(name = "Path",short = 'i', long = "ignore")]
            pub ignore_path: Option<std::path::PathBuf>,

            /// Add a tag to the assets with the given name
            #[arg(name = "Tag name",short = 't', long = "tag")]
            pub tag_name: Option<String>,
    }

    /// Checkout assets in local workspace from a source like . When assets are checkout, they are locked by default when in centralized mode.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Checkout{
        /// Specific assets or complete directory structure can be checkout
        pub path: Option<std::path::PathBuf>,
        /// Option use to specify if a lock should be put on the assets, this will tell central to lock them. (On by default if centralized, WARNING: If not set to lock you should clone the workspace instead "gawires checkout --clone" if you dont want any conflict issues)
        #[clap(name = "Lock",short = 'l', long = "lock")]
        pub lock_remote: Option<bool>,
        /// Can be use to specify that the checkout must clone the assets instead of synchronizing them.(Will need to initiate a push request each time you try to synchronize)
        pub clone: Option<bool>,

    }
    /// Release checkout assets from local workspace.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Release{
    }
    /// Initialize a new local workspace that can be connected to a central workspace
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

    /// Change or create content layout for the workspace (This enables to switch between different layout for the same workspace)
    #[derive(Debug, PartialEq, Parser)]
    pub struct Layout{
    }

    /// Used in a distributed workspace to push local changes to remote, will use Sync in centralized.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Push{
    }

    /// Used in a distributed workspace to pull localy changes from remote, will use Sync in centralized.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Pull{
    }

    /// Commands to manage workspace, change or create workspace layout and other operations.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Workspace{
    }

    /// Clone a remote workspace locally, it will clone a complete copy of it.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Clone{
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

    /// Commands related to Project management
    #[derive(Debug, PartialEq, Parser)]
    pub struct Project{
    }

    /// Install a Gawire extension in current workspace.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Install{
    }

    /// Uninstall a Gawire extension in current workspace.
    #[derive(Debug, PartialEq, Parser)]
    pub struct Uninstall{
    }

    /// Commands related to Assets management
    #[derive(Debug, PartialEq, Parser)]
    pub struct Asset{
    }
