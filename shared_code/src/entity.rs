use serde::{Serialize, Deserialize};
use slotmap::{SlotMap, SecondaryMap, DefaultKey};
use cgmath::*;
pub type EntityId = DefaultKey;
use std::ops::Mul;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entity {
    pub component_mask: u64,
    pub id: EntityId,
    pub transform: Matrix4<f32>,
    pub desired_position: Vector3<f32>,
    pub is_moving: bool
}

impl Entity {

    pub fn new() -> Entity {
        Entity {
            component_mask: 0,
            id: DefaultKey::default(),
            transform: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
            desired_position: Vector3::new(0.0, 0.0, 0.0),
            is_moving: false
        }
    }

    pub fn position(&self) -> Vector3<f32> {
        return Vector3::new(self.transform.w.x, self.transform.w.y, self.transform.w.z);
    }

    pub fn move_entity(&mut self, amount: Vector3<f32>) {
        self.transform = self.transform.mul(Matrix4::from_translation(Vector3::new(amount.x, amount.y, amount.z)));
    }
}