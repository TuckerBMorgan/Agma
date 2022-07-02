use bincode::{Decode, Encode};

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct EntityComponent {
    pub id: usize,
    pub in_use: bool
}

impl EntityComponent {
    pub fn new(id: usize) -> EntityComponent {
        EntityComponent {
            id,
            in_use: true
        }
    }
}