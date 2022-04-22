use bincode::{Decode, Encode};

/// 
#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub enum AutoAttackState {
    Windup,
    Firing,
    Recovery
}

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct AutoAttackComponent {
    pub state: AutoAttackState,
    pub target: usize,
    pub length_of_auto_attack: f32,
    pub current_progress: f32,
    pub attack_speed: f32
}