use crate::*;
use cgmath::*;

#[derive(Copy, Clone, Debug)]
pub struct DamageRune {
    pub target: usize,
    pub amount: usize
}

impl DamageRune {
    pub fn new(target: usize, amount: usize) -> DamageRune {
        DamageRune {
            target,
            amount
        }
    }
}

impl Into<Rune> for DamageRune {
    fn into(self) -> Rune {
        Rune::DamageRune(Box::new(self))
    }
}