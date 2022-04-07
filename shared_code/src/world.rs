use crate::*;
use cgmath::*;
use serde::{Serialize, Deserialize};
use std::ops::Mul;


use slotmap::{SlotMap, SecondaryMap, DefaultKey};
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldMouseState {
    pub button_down: bool,
    pub world_pos: Vector3<f32>
}

impl WorldMouseState {
    pub fn new(mouse_state: &MouseState) -> WorldMouseState {
        WorldMouseState {
            button_down:mouse_state.button_down,
            world_pos: Vector3::new(mouse_state.x, mouse_state.y, mouse_state.z)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub frame_number: usize,
    pub entities: Vec<Entity>,
    pub inputs: Vec<u8>,
    pub click_inputs: Vec<WorldMouseState>
}

impl World {
    pub fn new() -> World {
        World {
            frame_number: 0,
            entities: vec![],
            inputs: vec![],
            click_inputs: vec![]
        }
    }
    pub fn movement_system(&mut self) {
        if self.click_inputs.len() == 0 {
            return;
        }

        let mouse_event = self.click_inputs.remove(0);
        if mouse_event.button_down == false {
            return;
        }

        for entity in self.entities.iter_mut() {
            entity.is_moving = true;
            entity.desired_position = mouse_event.world_pos;
        }

        for entity in self.entities.iter_mut() {
            if entity.is_moving {
                //Direction we want the character to move in
                let direction = (entity.position() - entity.desired_position).normalize();
                entity.move_entity(direction * 0.1f32);
                println!("{:?}", direction);
            }
        }
    }

    pub fn client_tick(&mut self) {
        self.movement_system();
    }

    pub fn tick(&mut self) {
        self.frame_number += 1;
        self.movement_system();
    }

    pub fn post_tick(&mut self) {
    }
}