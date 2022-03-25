use crate::*;

use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug, Default)]
pub struct Entity {
    pub id: u32,
    pub pos: Vec3
}

impl Entity {
    pub fn tick(&mut self) { 
        self.pos.z += 1.0f32;
    }
}