use crate::*;

use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug, Default)]
pub struct PlayerInput {
    input_values: u32
}

impl PlayerInput {
    pub fn new(input_values: u32) -> PlayerInput {
        PlayerInput {
            input_values
        }
    }
}


#[derive(Encode, Decode, PartialEq, Debug, Default)]
pub struct World {
    pub frame_number: usize,
    pub entities: Vec<Entity>,
    pub inputs: Vec<PlayerInput>
}

impl World {
    pub fn tick(&mut self) {
        self.frame_number += 1;
        for entity in self.entities.iter_mut() {
            if self.inputs.len() > 0 {
                entity.pos.z += 1.0f32;
            }
        }
    }

    pub fn add_input(&mut self, input: PlayerInput) {
        self.inputs.push(input);
    }

    pub fn post_tick(&mut self) {
        self.inputs.clear();
    }
}