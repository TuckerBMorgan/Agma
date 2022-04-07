use cgmath::*;
use crate::*;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ChampionComponent {
    pub is_moving: bool,
    pub desired: Vector3<f32>
}

impl ChampionComponent {
    pub fn new() -> ChampionComponent {
        ChampionComponent {
            is_moving: false,
            desired: Vector3::new(0.0, 0.0, 0.0)
        }
    }
}

impl_component!(ChampionComponent, ChampionComponent);