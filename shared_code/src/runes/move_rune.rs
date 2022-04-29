use crate::*;
use cgmath::*;

#[derive(Copy, Clone, Debug)]
pub struct MoveRune {
    pub entity: usize,
    pub desired_amount: Vector3<f32>
}

impl MoveRune {
    pub fn new(entity: usize, desired_amount: Vector3<f32>) -> MoveRune {
        MoveRune {
            entity,
            desired_amount
        }
    }
}

impl Into<Rune> for MoveRune {
    fn into(self) -> Rune {
        Rune::MoveRune(Box::new(self))
    }
}