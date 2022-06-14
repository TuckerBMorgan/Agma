use bincode::{Decode, Encode};

use cgmath::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct AutoAttackState {
    pub target: usize,
    pub progress: usize
}

impl AutoAttackState {
    pub fn new(target: usize, progress: usize) -> AutoAttackState {
        AutoAttackState {
            target,
            progress
        }
    }
}

/// An enum used to represent the state of the character
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CharacterState {
    Idle,
    Moving(Vector3<f32>),
    AutoAttacking(AutoAttackState)
}

impl bincode::Encode for CharacterState {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        match self {
            CharacterState::Idle => {
                // Doing this to add a type to the number
                // so it will have a fixed size
                // we also need to stuff the decoder with fake data
                // rust structs are as large as their larger member
                let tag : u32 = 0;                
                bincode::Encode::encode(&tag, encoder)?;
            },
            CharacterState::Moving(location) => {
                let tag : u32 = 1;
                bincode::Encode::encode(&tag, encoder)?;
                bincode::Encode::encode(&location.x, encoder)?;
                bincode::Encode::encode(&location.y, encoder)?;
                bincode::Encode::encode(&location.z, encoder)?;
            },
            CharacterState::AutoAttacking(autoattackstate) => {
                let tag : u32 = 2;
                bincode::Encode::encode(&tag, encoder)?;
                bincode::Encode::encode(&autoattackstate.target, encoder)?;
                bincode::Encode::encode(&autoattackstate.progress, encoder)?;
            }
        }
        Ok(())
    }
}

impl bincode::Decode for CharacterState {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        let tag : u32 = bincode::Decode::decode(decoder).unwrap();

        if tag == 0 {
            return Ok(CharacterState::Idle);
        }
        else if tag == 1 {
            return Ok(CharacterState::Moving(Vector3::new(
                bincode::Decode::decode(decoder).unwrap(), 
                bincode::Decode::decode(decoder).unwrap(), 
                bincode::Decode::decode(decoder).unwrap())
            ));
        }
        else if tag == 2 {            
            return Ok(CharacterState::AutoAttacking(
                AutoAttackState {
                    target: bincode::Decode::decode(decoder).unwrap(),
                    progress: bincode::Decode::decode(decoder).unwrap()
                }
            ));
        }
        panic!("This tag has not yet been implemented for Movemenet state decoding, add it to the if statement above");
    }
}

/// A component on most drivable entities, champions, minions
/// so that the character system to issue runes
#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct CharacterStateComponent {
    pub character_state: CharacterState
}

impl CharacterStateComponent {
    pub fn new() -> CharacterStateComponent {
        CharacterStateComponent {
            character_state: CharacterState::Idle
        }
    }
}