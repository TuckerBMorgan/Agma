// Example IDL file for our monster's schema.
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ToPlayerMessageType {
    UpdateWorld,
    StateWorld
}
 
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

pub const PLAYER_MESSAGE_TYPE_BASE : u8 = 0;

pub const INPUT_WINDOW_MESSAGE : u8 = PLAYER_MESSAGE_TYPE_BASE + 1;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[repr(C)]
pub struct InputWindowMessage {
    pub message_type: u8,
    pub input_messages: Vec<u8>
}

impl InputWindowMessage {
    pub fn new(inputs: Vec<u8>) -> InputWindowMessage {
        InputWindowMessage {
            message_type: INPUT_WINDOW_MESSAGE,
            input_messages: inputs
        }
    }
}

pub const AWK_FRAME_MESSAGE : u8 = INPUT_WINDOW_MESSAGE + 1;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[repr(C)]
pub struct AwkFrameMessage {
    pub message_type: u8,
    pub frame_number: usize
}

impl AwkFrameMessage {
    pub fn new(frame_number: usize) -> AwkFrameMessage {
        AwkFrameMessage {
            message_type: AWK_FRAME_MESSAGE,
            frame_number
        }
    }
}

