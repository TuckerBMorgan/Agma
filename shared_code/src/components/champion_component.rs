use bincode::{Decode, Encode};
use crate::*;

/// Champion Component
/// Used as a proxy for a player, and a look up for which 
/// champion they are, and what inputs the server is aware of at the moment
#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct ChampionComponent {
    pub champion_index: u8,
    pub desired_inputs: [MouseState;16],
    pub current_input_to_use: usize
}

impl ChampionComponent {
    pub fn new(champion_index: u8) -> ChampionComponent {
        ChampionComponent {
            champion_index,
            desired_inputs: [MouseState::default();16],
            current_input_to_use: 0
        }
    }

    /// Gets and removes the furthest in the past input we are aware of
    /// is refilled by the PlayerConnectionComponent and System
    pub fn get_current_input(&mut self) -> Option<MouseState> {
        if self.current_input_to_use < 16 {
            self.current_input_to_use += 1;        
            return Some(self.desired_inputs[self.current_input_to_use - 1].clone());
        }
        else {
            return None;
        }
    }
}
