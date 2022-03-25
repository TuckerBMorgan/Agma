use crate::*;

use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug, Default)]
pub struct Entity {
    pub pos: Vec3
}