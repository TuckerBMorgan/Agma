// Example IDL file for our monster's schema.
use cgmath::*;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
 
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UpdateWorldMessage {
    pub message_type: ToPlayerMessageType,
    pub current_frame_number: usize,
    pub delta_frame_number: usize,
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

