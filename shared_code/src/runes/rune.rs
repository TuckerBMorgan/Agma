use cgmath::*;
use std::collections::HashMap;

use crate::*;

/// A wrapping structure for all runes
/// essentially a tagged structure allowing
/// runes heap allocated and stored in generic structures
/// but then turned into concrete types in a safe manner
/// Careful: sizeof(AnInstance) is equal to the largest 
/// single member of the enum
/// so if we have Rune::(usize)
/// and Rune::(usize, size)
/// Rune::(usize) will still be of size (usize, usize)
pub enum Rune {
    MoveRune(Box<MoveRune>),
    DamageRune(Box<DamageRune>)
}

impl Rune {
    /// Will return a usize for each of the
    /// allowing them to be ordered when needed
    pub fn value(&self) -> usize {
        match self {
            Rune::MoveRune(_) => {
                return 0;
            },
            Rune::DamageRune(_) => {
                return 1;
            }
        }
    }
}

/// The event system for the ECS
/// it will collect runes until it is drained
pub struct RuneSystem {
    //This will need to change
    current_runes: HashMap<usize, Vec<Rune>>

}

impl RuneSystem {
    pub fn new() -> RuneSystem {
        RuneSystem {
            current_runes: HashMap::new()
        }
    }

    /// pushes a rune into the back of the rune stack
    /// for that type of rune
    /// # Arguments
    /// * 'rune' - The rune that will be added
    pub fn add_rune(&mut self, rune: Rune) {
        if self.current_runes.contains_key(&rune.value()) == false{
            self.current_runes.insert(rune.value(), vec![]);
        }
        self.current_runes.get_mut(&rune.value()).unwrap().push(rune);
    }

    pub fn resolve_world_state(&mut self, world: &mut World) {
        
        let movement_runes = self.current_runes.remove(&0).unwrap();
        self.handle_move_runes(movement_runes, world);

        let damage_runes = self.current_runes.remove(&1).unwrap();
        self.handle_damage_runes(damage_runes, world);
    }

    pub fn handle_move_runes(&mut self, mut movement_runes: Vec<Rune>, world: &mut World) {
        let movement_runes : Vec<MoveRune> = movement_runes.iter_mut().map(
            |x|
            match x {
                Rune::MoveRune(move_rune) => {
                    return *move_rune.to_owned();
                },
                _ => {
                    panic!("COME AND GET ME");
                }
            }
        ).collect();

        let transforms = world.borrow_component_vec::<TransformComponent>().unwrap();

        // Find all of the places we want to move objects to
        for rune in movement_runes.iter() {
            let current_transform = transforms[rune.entity];
            match current_transform {
                Some(mut current_transform) => {
                    current_transform.move_character(rune.desired_amount);
                },
                None => {}
            }
        }
    }

    pub fn handle_damage_runes(&mut self, mut damage_runes: Vec<Rune>, world: &mut World) {
        let damage_runes : Vec<MoveRune> = damage_runes.iter_mut().map(
            |x|
            match x {
                Rune::DamageRune(damage_rune) => {
                    return *damage_rune.to_owned();
                },
                _ => {
                    panic!("COME AND GET ME");
                }
            }
        ).collect();

        let health_components = world.borrow_component_vec::<HealthComponent>().unwrap();
        // Find all of the places we want to move objects to
        for rune in damage_runes.iter() {
            let current_health = health_components[rune.target];
            match current_health {
                Some(mut current_health) => {
                    current_health.do_damage(rune.amount);
                },
                None => {}
            }
        }
    }
}