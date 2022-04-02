use cgmath::*;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AttributeComponent {
    pub move_speed: f32
}

impl AttributeComponent {
    pub fn new(move_speed: f32) -> AttributeComponent {
        AttributeComponent {
            move_speed
        }
    }
}