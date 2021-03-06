use bincode::{Decode, Encode};

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub enum MovementType {
    Location,
    AttackMove(usize, usize)
}

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct MovementStateComponent {
    pub is_moving: bool,
    pub destination_x: i64,
    pub destination_y: i64,
    pub can_move: bool,
    pub move_speed: usize,
    pub current_move_speed: usize,
    pub movement_type: MovementType
}

impl MovementStateComponent {
    pub fn new(move_speed: usize) -> MovementStateComponent {
        MovementStateComponent {
            is_moving: false,
            destination_x: 0,
            destination_y: 0,
            can_move: true,
            move_speed,
            current_move_speed: move_speed,
            movement_type: MovementType::Location
        }
    }

    pub fn stop_moving(&mut self, ) {

    }

    pub fn start_moving(&mut self, destination_x: i64, destination_y: i64) {
        self.is_moving = true;
        self.destination_x = destination_x;
        self.destination_y = destination_y;
        self.movement_type = MovementType::Location;
    }
    
    pub fn start_attack_moving(&mut self, character: usize, maximum_range: usize) {
        self.is_moving = true;
        self.movement_type = MovementType::AttackMove(character, maximum_range);
    }
}