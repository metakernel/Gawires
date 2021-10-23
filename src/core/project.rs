
use super::workspace::Workspace;


/// Data structure for storing project information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Project {
    /// Project name.
    pub name: String,
    /// Project description.
    pub description: String,
    /// Project owner.
    pub owner: Member,
    /// Project workplaces.
    pub workspaces: Vec<Workspace>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Member{
    member_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Remote{
    remote_url: String,
}