pub mod entity_component;

use uuid::Uuid;

// Représente un UUID pour chaque entité
type EntityId = Uuid;
/// 
pub struct Entity{
    pub id: EntityId,
    pub components: Vec<Uuid>,
    
}