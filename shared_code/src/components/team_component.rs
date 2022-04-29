use bincode::{Decode, Encode};

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct TeamComponent {
    pub team: usize
}

impl TeamComponent {
    pub fn new(team: usize) -> TeamComponent {
        TeamComponent {
            team
        }
    }
}