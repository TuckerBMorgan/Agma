use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::*;
#[derive(Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
#[repr(u64)]
pub enum ComponentType {
    TransformComponent = 1,//The first one needs to be 1
    ChampionComponent,
    AttributeComponent
}

//A component is a tagged pointer to a particular component
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Component {
    TransformComponent(Box<TransformComponent>),
    ChampionComponent(Box<ChampionComponent>),
    AttributeComponent(Box<AttributeComponent>)
}

impl Component {
    pub fn component_type(&self) -> ComponentType {
        match self {
            Component::TransformComponent(_) => {
                ComponentType::TransformComponent
            },
            Component::ChampionComponent(_) => {
                ComponentType::ChampionComponent
            },
            Component::AttributeComponent(_) => {
                ComponentType::AttributeComponent
            }
        }
    }
}