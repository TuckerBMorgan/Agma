use crate::*;
use cgmath::*;
use serde::{Serialize, Deserialize};
use std::ops::Mul;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
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


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct World {
    pub frame_number: usize,
    pub entities: Vec<Entity>,
    pub transforms: Vec<TransformComponent>,
    pub input: u8
}

impl World {
    pub fn tick(&mut self) {
        self.frame_number += 1;
        for transform_componenet in self.transforms.iter_mut() {
            if self.input > 0 {
                if self.input & 1 > 0 {
                    transform_componenet.transform = transform_componenet.transform.mul(Matrix4::from_translation(Vector3::new(0.0f32, 0.0, 1.0)));
                }
                if self.input & 2 > 0 {
                    transform_componenet.transform = transform_componenet.transform.mul(Matrix4::from_translation(Vector3::new(0.0f32, 0.0, -1.0)));
                }
                if self.input & 4 > 0 {
                    transform_componenet.transform = transform_componenet.transform.mul(Matrix4::from_translation(Vector3::new(1.0f32, 0.0, 0.0)));
                }
                if self.input & 8 > 0{
                    transform_componenet.transform = transform_componenet.transform.mul(Matrix4::from_translation(Vector3::new(-1.0f32, 0.0, 0.0)));
                }
            }
        }
    }

    pub fn post_tick(&mut self) {
      //  self.input = 0;
    }
}