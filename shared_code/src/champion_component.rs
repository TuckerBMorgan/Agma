use cgmath::*;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ChampionComponent {
    pub is_moving: bool,
    pub desired_x: f32,
    pub desired_y: f32
}

impl ChampionComponent {
    pub fn new() -> ChampionComponent {
        ChampionComponent {
            is_moving: false,
            desired_x: 0.0, 
            desired_y: 0.0
        }
    }
}