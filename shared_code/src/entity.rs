use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Entity {
    pub id: u32
}
