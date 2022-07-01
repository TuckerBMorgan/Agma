use bincode::{Decode, Encode};

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct AttackStateComponent {
    pub is_attacking: bool,
    pub range: usize,
    pub current_target: usize,
    pub cooldown_timer: usize,
    pub current_cooldown: usize,
    pub channel_timer: usize,
    pub current_channel: usize,
    pub location_x: i64,
    pub location_y: i64
}

impl AttackStateComponent {
    pub fn new(cooldown_timer: usize, range: usize, channel_timer: usize) -> AttackStateComponent {
        AttackStateComponent {
            is_attacking: false,
            range,
            current_target: 0,
            cooldown_timer,
            current_cooldown: 0,
            channel_timer,
            current_channel: 0,
            location_x: 0, 
            location_y: 0
        }
    }

    pub fn start_attacking(&mut self, target: usize, location_x: i64, location_y: i64) {
        self.is_attacking = true;
        self.current_target = target;
        self.location_x = location_x;
        self.location_y = location_y;
    }
}