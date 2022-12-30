/// In Gawires a workspace is a slice of a project that can be worked on independently, each workspace can act on a subset of the project assets.
/// From a local perspective a gawires Workspace is a folder that contains assets and a .gaw folder that contains the workspace configuration and metadata.

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