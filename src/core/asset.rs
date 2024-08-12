pub mod version_graph;
pub mod entity;
pub mod asset_proxy;
pub mod asset_tag;

use uuid::Uuid;
use version_graph::VersionGraph;

// Représente un UUID pour chaque entité
type AssetId = Uuid;

pub struct Asset {
    pub id: AssetId,
    pub name: String,
    pub version_graph: VersionGraph,
    
}