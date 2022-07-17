use bincode::{Decode, Encode};
use crate::*;


pub const MOVEMENT_ABILITY_INDEX : usize = 0;

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct AbilityComponent {
    pub ability_ids: [AbilityInstanceId;4],
    pub active_ability: usize
}

impl AbilityComponent {
    pub fn new(ability_ids: [AbilityInstanceId;4]) -> AbilityComponent {
        AbilityComponent {
            ability_ids,
            active_ability: 1 //It is 1, since your "movement_ability(index 0)" is always active as the default last action
        }
    }
}