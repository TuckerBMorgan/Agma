use cgmath::*;
use std::collections::HashMap;

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
    MoveRune(Box<MoveRune>)
}

impl Rune {
    /// Will return a usize for each of the
    /// allowing them to be ordered when needed
    pub fn value(&self) -> usize {
        match self {
            Rune::MoveRune(_) => {
                return 0;
            }
        }
    }
}

/// The event system for the ECS
/// it will collect runes until it is drained
pub struct RuneSystem {
    //This will need to change
    current_runes: HashMap<usize, Vec<Rune>>,

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
}