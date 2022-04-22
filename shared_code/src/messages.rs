// Example IDL file for our monster's schema.
use cgmath::*;
use bincode::{Decode, Encode};
#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(u8)]
pub enum ToPlayerMessageType {
    UpdateWorld,
    StateWorld
}
/*
impl ToPlayerMessageType {
    pub fn to_u8(&self) -> u8 {
        match *self {
            ToPlayerMessageType::UpdateWorld => {
                
            }
        }
    }
}
*/
 
/// This is used to update the client on the state of the 
/// world, either partially or entirely
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct UpdateWorldMessage {
    /// The kind of message, DeltaUpdate, WholeUpdate
    pub message_type: ToPlayerMessageType,
    /// What server frame this will decode into to
    pub current_frame_number: usize,
    /// What server frame that the player has awked
    /// that we would need to use to build a full state
    pub delta_frame_number: usize,
    /// the actual data that we will diff with
    pub data: Vec<u8>
}

impl UpdateWorldMessage {
    pub fn new(current_frame_number: usize, delta_frame_number: usize, data: Vec<u8>) -> UpdateWorldMessage {
        UpdateWorldMessage {
            message_type: ToPlayerMessageType::StateWorld,
            current_frame_number,
            delta_frame_number,
            data
        }
    }
}

/// An enum used to tag a message from the player 
/// to the server what kind it is
#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(u8)]
pub enum PlayerToServerMessage {
    KeyboardAction,
    MouseAction,
    AwkFrameMessage
}

impl PlayerToServerMessage {
    pub fn to_u8(&self) -> u8 {
        match self {
            PlayerToServerMessage::AwkFrameMessage => {
                return 0;
            }
            PlayerToServerMessage::MouseAction => {
                return 1;                
            }
            PlayerToServerMessage::KeyboardAction => {
                return 2;
            }
        }
    }

    pub fn from_u8(value: u8) -> PlayerToServerMessage {
        match value {
            0 => {
                return PlayerToServerMessage::AwkFrameMessage;
            },
            1 => {
                return PlayerToServerMessage::MouseAction;
            }
            2 => {
                return PlayerToServerMessage::KeyboardAction;
            },
            _ => {
                panic!("unknow PlayerToServerMessage decode value {}", value);
            }
        }
    }
}


/// The snapshot of the mouse state for a frame
#[derive(Encode, Decode, PartialEq, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct MouseState {
    pub button_down: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl MouseState {
    pub fn new(mouse_down: bool, world_pos: Vector3<f32>) -> MouseState {
        MouseState {
            button_down: mouse_down,
            x: world_pos.x,
            y: world_pos.y,
            z: world_pos.z
        }
    }
}

/// A collection of mouse snapshots, what is actually sent to the server
/// it is a sliding window, so if a message is lost the server
/// can recover it from the next message
#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(C)]
pub struct  MouseActionMessage {
    pub message_type: PlayerToServerMessage,
    pub destinations: Vec<MouseState>
}

impl MouseActionMessage {
    pub fn new(destinations: Vec<MouseState>) -> MouseActionMessage {
        MouseActionMessage {
            message_type: PlayerToServerMessage::MouseAction,
            destinations
        }
    }
}

/// A message use to the tell the player the state of the keyboard is on
/// a collection of frames
#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(C)]
pub struct KeyboardActionMessage {
    pub message_type: PlayerToServerMessage,
    pub input_messages: Vec<u8>
}

impl KeyboardActionMessage {
    pub fn new(inputs: Vec<u8>) -> KeyboardActionMessage {
        KeyboardActionMessage {
            message_type: PlayerToServerMessage::KeyboardAction,
            input_messages: inputs
        }
    }
}


/// A message used to the tell the server that a frame got to the
/// client so it can be used to build diffs later on
#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(C)]
pub struct AwkFrameMessage {
    pub message_type: PlayerToServerMessage,
    pub frame_number: usize
}

impl AwkFrameMessage {
    pub fn new(frame_number: usize) -> AwkFrameMessage {
        AwkFrameMessage {
            message_type: PlayerToServerMessage::AwkFrameMessage,
            frame_number
        }
    }
}

