use crate::*;

use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug, Default)]
pub struct World {
    pub frame_number: usize,
    pub entities: Vec<Entity>
}
