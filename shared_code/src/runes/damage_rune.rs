use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct DamageRune {
    pub source: usize,
    pub target: usize,
    pub amount: usize
}

impl DamageRune {
    pub fn new(source: usize, target: usize, amount: usize) -> DamageRune {
        DamageRune {
            source,
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