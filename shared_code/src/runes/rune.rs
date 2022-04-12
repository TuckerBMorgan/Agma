use cgmath::*;
use crate::*;
use log::{info};

pub enum RuneType {
    MoveRune(Box<MoveRune>)
}

pub struct RuneSystem {
    //This will need to change
    current_runes: Vec<MoveRune>
}

impl RuneSystem {
    pub fn new() -> RuneSystem {
        RuneSystem {
            current_runes: vec![]
        }
    }

    pub fn add_runes(&mut self, runes: Vec<MoveRune>) { 
        self.current_runes.extend(runes);
    }

    pub fn execute_current_stack(&mut self, world: &mut World) {
        for rune in self.current_runes.drain(..) {
            rune.execute(world);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MoveRune {
    entity: usize,
    desired_amount: Vector3<f32>
}

impl MoveRune {
    pub fn new(entity: usize, desired_amount: Vector3<f32>) -> MoveRune {
        MoveRune {
            entity,
            desired_amount
        }
    }

    fn execute(&self, world: &mut World)  {

        match world.entities[self.entity].entity_state {
            EntityState::Moving(desired_position) => {
                world.entities[self.entity].move_entity(self.desired_amount);
                let distance = (desired_position - world.entities[self.entity].position()).magnitude().abs();
                if distance < 0.1 {
                    world.entities[self.entity].entity_state = EntityState::Idle;
                }
            },
            EntityState::Idle => {

            }
        }
    }
}