use bincode::{Decode, Encode};
use rand::Rng;

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub enum ZomebieState {
    Idle{walk_timer: usize},
    Fighting{target: usize},
    Hunting{location_x: i64, location_y: i64}
}

pub const ZOMBIE_WALK_TIMER_MAX : usize = 240;
/// Champion Component
/// Used as a proxy for a player, and a look up for which 
/// champion they are, and what inputs the server is aware of at the moment
#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct ZomebieControllerComponent {
    pub state: ZomebieState
}

impl ZomebieControllerComponent {
    pub fn new() -> ZomebieControllerComponent {
        let mut rng = rand::thread_rng();
        ZomebieControllerComponent {
            state: ZomebieState::Idle{walk_timer: rng.gen_range(0..10)}
        }
    }
}
