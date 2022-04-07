use cgmath::*;
use crate::*;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct TransformComponent {
    pub transform: Matrix4<f32>
}

impl TransformComponent {
    pub fn new(transform: Matrix4<f32>) -> TransformComponent {
        TransformComponent {
            transform
        }
    }

    pub fn position(&self) -> Vector3<f32> {
        return Vector3::new(self.transform.w.x, self.transform.w.y, self.transform.w.z);
    }


}

impl_component!(TransformComponent, TransformComponent);