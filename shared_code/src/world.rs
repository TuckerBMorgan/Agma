use crate::*;
use cgmath::*;
use serde::{Serialize, Deserialize};

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

    pub fn movement_system(&mut self, delta_time: f32) -> Vec<MoveRune> {
        if self.click_inputs.len() != 0 {
            let mouse_event = self.click_inputs.remove(0);
            if mouse_event.button_down == true {
                for entity in self.entities.iter_mut() {
                    entity.entity_state = EntityState::Moving(mouse_event.world_pos);
                }
            }
        }
        let mut return_runes = vec![];
        for entity in self.entities.iter() {
            match entity.entity_state {
                EntityState::Moving(desired_position) => {
                    let direction = (desired_position - entity.position()).normalize();
                    let move_rune = MoveRune::new(0, direction * entity.move_speed);
                    return_runes.push(move_rune);
                },
                EntityState::Idle => {

                }
            }
        }

        return return_runes;
    }

    pub fn client_tick(&mut self, delta_time: f32) {
       // self.movement_system(delta_time);
    }

    //TODO: tick, this might be it's own system, and remove this
    pub fn tick(&mut self) {
        self.frame_number += 1;
    }

    pub fn post_tick(&mut self) {
        
    }
}