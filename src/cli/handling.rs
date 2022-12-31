use super::commands::Commands;
// Its a nasty implementation for the moment, the printlines are useless placeholders and log should be done by the executing functions
pub fn handle_cmds(cmds: Commands) {
    match &cmds {
        Commands::Add(add) => {
            // if the user put nothing to add tell him this is an error
            if add.path.is_none() && !add.all {
                println!("No asset specified, use --help to get some more informations");
            } 
            // The user wants to stage all assets in the current workspace
            else if add.path.is_none() && add.all {
                println!("Staging all changes and untracked files to workspace");
            }
            // The user wants to stage an asset from path or using Fileglobs and other wildcards
            else if add.path.is_some() && !add.all {
                let path = &add.path;
                let is_dir = path.as_ref().expect("Path is not correct").is_dir();

                if is_dir {
                    println!("Adding all files under {} to workspace", path.as_ref().unwrap().display());
                } else {println!("Adding {} to workspace", path.as_ref().unwrap().display());}
            }

            // The user wants to stage all assets with the following path
            else if add.path.is_some() && add.all {
                let path = &add.path;
                let is_dir = path.as_ref().expect("Path is not correct").is_dir();

                 if is_dir {
                     println!("Adding all files under {} for tracking", path.as_ref().unwrap().display());
                    } else {println!("Adding all occurence of {} for tracking",  path.as_ref().unwrap().display());}
            }
            else {
                println!("Wrong Commands specified, use --help to get some more informations");
            }
        }
        Commands::Checkout(checkout) => {
            println!("Checking out");
        }
        Commands::Release(release) => {
            println!("Releasing");
        }
        Commands::Init(_) => todo!(),
        Commands::Sync(_) => todo!(),
        Commands::Status(_) => todo!(),
        Commands::Layout(_) => todo!(),
        Commands::Push(_) => todo!(),
        Commands::Pull(_) => todo!(),
        Commands::Clone(_) => todo!(),
        Commands::Workspace(_) => todo!(),
        Commands::Preview(_) => todo!(),
        Commands::Reset(_) => todo!(),
        Commands::Remove(_) => todo!(),
        Commands::Branch(_) => todo!(),
        Commands::Wire(_) => todo!(),
        Commands::Connect(_) => todo!(),
        Commands::Filter(_) => todo!(),
        Commands::Rebase(_) => todo!(),
        Commands::Merge(_) => todo!(),
        Commands::Clean(_) => todo!(),
        Commands::Project(_) => todo!(),
        Commands::Install(_) => todo!(),
        Commands::Uninstall(_) => todo!(),
        Commands::Asset(_) => todo!(),
    }
}