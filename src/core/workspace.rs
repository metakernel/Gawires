use crate::core::repository::{Repository, Remote};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace<'a> {
    pub name: String,
    pub owning_project: Repository<'a>,
    pub workspace_type: WorkspaceType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkspaceType {
    Root,
    Sub,
}

pub enum WorkspaceError {
    WorkspaceNotFound,
}

pub enum WorkspaceMode {
    Listen,
    Local,
}

pub enum LocalWorkspaceState {
    Connected(Remote),
    Offline,
}