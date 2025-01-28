/// In Gawires a Workpod is a local representation of a remote Workspace, it is a folder that contains assets and a .gaw folder that contains the Workpod configuration and metadata.

use crate::project::{Project, Remote};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workpod<'a> {
    pub name: String,
    pub owning_project: Project<'a>,
    pub workpod_type: WorkpodType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkpodType {
    Sync, // Sync Workpod is a Workpod that is connected to a remote Workspace
    Clone, // Clone Workpod is a Workpod that is a copy of a remote Workspace
}

pub enum WorkpodError {
    WorkpodNotFound,
    WorkpodAlreadyExists, // Workpod already exists in the project
    WorkpodNotConnected, // Workpod is not connected to a remote Workspace
    WorkpodNotCloned,
}

pub enum WorkpodState {
    Connected(Remote),
    Offline,
}