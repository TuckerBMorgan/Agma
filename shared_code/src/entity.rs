use serde::{Serialize, Deserialize};
use cgmath::*;
use std::ops::Mul;
use crate::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum EntityState {
    Idle,
    Moving(Vector3<f32>)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entity {
    pub component_mask: u64,
    pub entity_state: EntityState,
    pub transform: Matrix4<f32>,
    pub move_speed: f32
}

impl Entity {

    pub fn new() -> Entity {
        Entity {
            component_mask: 0,
            entity_state: EntityState::Idle,
            transform: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
            move_speed: 0.1
        }
    }

    pub fn position(&self) -> Vector3<f32> {
        return Vector3::new(self.transform.w.x, self.transform.w.y, self.transform.w.z);
    }

    pub fn move_entity(&mut self, amount: Vector3<f32>) {
        self.transform = self.transform.mul(Matrix4::from_translation(Vector3::new(amount.x, amount.y, amount.z)));
    }
}