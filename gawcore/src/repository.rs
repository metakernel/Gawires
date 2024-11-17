use super::workpod::Workpod;

use super::user::UserStamp;


/// Data structure for storing project information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository<'a> {
    /// Repository name.
    pub name: String,
    /// Repository description.
    pub description: String,
    /// Repository owner.
    pub owner: UserStamp,
    /// Ref of repository workplaces.
    pub workpods: Vec<&'a Workpod<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remote{
    remote_url: String,
}