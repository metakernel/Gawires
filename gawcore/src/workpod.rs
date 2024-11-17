/// In Gawires a Workpod is a slice of a project that can be worked on independently, each Workpod can act on a subset of the project assets.
/// From a local perspective a gawires Workpod is a folder that contains assets and a .gaw folder that contains the Workpod configuration and metadata.

use crate::repository::{Repository, Remote};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workpod<'a> {
    pub name: String,
    pub owning_project: Repository<'a>,
    pub workpod_type: WorkpodType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkpodType {
    Central,
    Clone,
}

pub enum WorkpodError {
    WorkpodNotFound,
}

pub enum WorkpodState {
    Connected(Remote),
    Offline,
}