// Example IDL file for our monster's schema.
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
    fn to_u8(&self) -> u8 {
        match self {
            PlayerToServerMessage::AwkFrameMessage => {
                return 1;
            }
            PlayerToServerMessage::MouseAction => {
                return 2;                
            }
            PlayerToServerMessage::KeyboardAction => {
                return 3;
            }
        }
    }

    fn from_u8(value: u8) -> PlayerToServerMessage {
        match value {
            1 => {
                return PlayerToServerMessage::AwkFrameMessage;
            },
            2 => {
                return PlayerToServerMessage::MouseAction;
            }
            3 => {
                return PlayerToServerMessage::KeyboardAction;
            },
            _ => {
                panic!("unknow PlayerToServerMessage decode value {}", value);
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[repr(C)]
pub struct MouseActionMessage {
    pub message_type: PlayerToServerMessage,
    pub destination_x: u32,
    pub destination_y: u32
}

impl MouseActionMessage {
    pub fn new(destination_x: u32, destination_y: u32) -> MouseActionMessage {
        MouseActionMessage {
            message_type: PlayerToServerMessage::MouseAction,
            destination_x,
            destination_y
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

