use super::project::{Project, Remote};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Workspace {
    pub name: String,
    pub owning_project: Project,
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
    Central,
    Local,
}

pub enum LocalWorkspaceState {
    Connected(Remote),
    Offline,
}