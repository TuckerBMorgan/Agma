use cgmath::*;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TransformComponent {
    pub transform: Matrix4<f32>
}

impl TransformComponent {
    pub fn new(transform: Matrix4<f32>) -> TransformComponent {
        TransformComponent {
            transform
        }
    }
}