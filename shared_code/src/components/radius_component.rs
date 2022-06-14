use bincode::{Decode, Encode};
use crate::*;

/// Radius Component
/// Represents the size of the entity in the world
/// all entities are just circles
#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct RadiusComponent {
    pub radius: f32
}

impl RadiusComponent {
    pub fn new(radius: f32) -> RadiusComponent {
        RadiusComponent {
            radius
        }
    }
}