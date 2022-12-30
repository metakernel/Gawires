use super::subcommand::Subcommand;
// Its a nasty implementation for the moment, the printlines are useless placeholders and log should be done by the executing functions
pub fn handle_subcmd(subcmd: Subcommand) {
    match subcmd {
        Subcommand::Add(add) => {
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
                let path = add.path.unwrap();
                let is_dir = path.is_dir();

                if is_dir {
                    println!("Adding all files under {} to workspace", path.display());
                } else {println!("Adding {} to workspace", path.display());}
            }

            // The user wants to stage all assets with the following path
            else if add.path.is_some() && add.all {
                let path = add.path.unwrap();
                let is_dir = path.is_dir();

                 if is_dir {
                     println!("Adding all files under {} for tracking", path.display());
                    }else {println!("Adding all occurence of {} for tracking", path.display());}
            }
            else {
                println!("Wrong subcommand specified, use --help to get some more informations");
            }
        }
        Subcommand::Checkout(checkout) => {
            println!("Checking out");
        }
        Subcommand::Release(release) => {
            println!("Releasing");
        }
        Subcommand::Init(_) => todo!(),
        Subcommand::Sync(_) => todo!(),
        Subcommand::Status(_) => todo!(),
        Subcommand::Channel(_) => todo!(),
        Subcommand::Push(_) => todo!(),
        Subcommand::Pull(_) => todo!(),
        Subcommand::Clone(_) => todo!(),
        Subcommand::Workspace(_) => todo!(),
        Subcommand::Preview(_) => todo!(),
        Subcommand::Reset(_) => todo!(),
        Subcommand::Remove(_) => todo!(),
        Subcommand::Branch(_) => todo!(),
        Subcommand::Tag(_) => todo!(),
        Subcommand::Wire(_) => todo!(),
        Subcommand::Connect(_) => todo!(),
        Subcommand::Filter(_) => todo!(),
        Subcommand::Rebase(_) => todo!(),
        Subcommand::Merge(_) => todo!(),
        Subcommand::Clean(_) => todo!(),
        Subcommand::Central(_) => todo!(),
        Subcommand::Install(_) => todo!(),
        Subcommand::Uninstall(_) => todo!(),
        Subcommand::Gist(_) => todo!(),
    }
}