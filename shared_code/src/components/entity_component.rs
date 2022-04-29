use bincode::{Decode, Encode};

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct EntityComponent {
    pub id: usize
}

impl EntityComponent {
    pub fn new(id: usize) -> EntityComponent {
        EntityComponent {
            id
        }
    }
}