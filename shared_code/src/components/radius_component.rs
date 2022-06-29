use bincode::{Decode, Encode};

/// Radius Component
/// Represents the size of the entity in the world
/// all entities are just circles
#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct RadiusComponent {
    pub radius: i64
}

impl RadiusComponent {
    pub fn new(radius: i64) -> RadiusComponent {
        RadiusComponent {
            radius
        }
    }
}