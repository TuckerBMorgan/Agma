use crate::*;

use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug, Default)]
pub struct PlayerInput {
    input_values: u8
}

impl PlayerInput {
    pub fn new(input_values: u8) -> PlayerInput {
        PlayerInput {
            input_values
        }
    }
}


#[derive(Encode, Decode, PartialEq, Debug, Default)]
pub struct World {
    pub frame_number: usize,
    pub entities: Vec<Entity>,
    pub input: u8
}

impl World {
    pub fn tick(&mut self) {
        self.frame_number += 1;
        for entity in self.entities.iter_mut() {
            if self.input > 0 {
                if self.input & 1 > 0 {
                    entity.pos.z += 1.0f32;
                }
                if self.input & 2 > 0 {
                    entity.pos.z -= 1.0f32;
                }
                if self.input & 4 > 0 {
                    entity.pos.x -= 1.0f32;
                }
                if self.input & 8 > 0{
                    entity.pos.x += 1.0f32;
                }
            }
        }
    }

    pub fn post_tick(&mut self) {
      //  self.input = 0;
    }
}