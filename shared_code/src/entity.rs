use serde::{Serialize, Deserialize};
use slotmap::{SlotMap, SecondaryMap, DefaultKey};
pub type EntityId = DefaultKey;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Entity {
    pub component_mask: u64,
    pub id: EntityId
}