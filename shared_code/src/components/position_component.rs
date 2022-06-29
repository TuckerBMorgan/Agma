use bincode::{Decode, Encode};

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct PositionComponent {
    pub x: i64,
    pub y: i64
}

impl PositionComponent {
    pub fn new(x: i64, y: i64) -> PositionComponent {
        PositionComponent {
            x,
            y
        }
    }

    pub fn update_position(&mut self, x: i64, y: i64) {
        self.x += x;
        self.y += y;
    }
}