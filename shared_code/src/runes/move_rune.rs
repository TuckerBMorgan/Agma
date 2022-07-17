use crate::*;
use cgmath::*;

#[derive(Copy, Clone, Debug)]
pub struct MoveRune {
    pub entity: usize,
    pub offset_x: i64,
    pub offset_y: i64
}

impl MoveRune {
    pub fn new(entity: usize, offset_x: i64, offset_y: i64) -> MoveRune {
        MoveRune {
            entity,
            offset_x,
            offset_y
        }
    }
}

impl Into<Rune> for MoveRune {
    fn into(self) -> Rune {
        Rune::MoveRune(Box::new(self))
    }
}