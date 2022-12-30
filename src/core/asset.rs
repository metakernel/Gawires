pub mod version_graph;

use crate::core::uuid::Uuid;
use version_graph::VersionGraph;

pub struct Asset {
    pub uuid: Uuid,
    pub name: String,
    pub version_graph: VersionGraph,
    
}