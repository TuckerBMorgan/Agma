use std::fmt::Debug;
use serde::{Serialize, Deserialize};
#[derive(Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
#[repr(u64)]
pub enum ComponentType {
    TransformComponent = 1,//The first one needs to be 1
}

pub trait Component<'a> : Debug + Deserialize<'a> + Serialize {
    fn component_type(&self) -> ComponentType;
}
