// Example IDL file for our monster's schema.
use bincode::{config, Decode, Encode};
 
#[derive(Encode, Decode, PartialEq, Debug)]
pub enum ToPlayerMessageType {
    UpdateWorld,
    StateWorld
}
 
#[derive(Encode, Decode, PartialEq, Debug)]
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

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum ToServerMessageType {
    AwkFrameMessage = 1,
    InputMessage = 2
}

#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(C)]
pub struct InputWindowMessage {
    pub message_type: ToServerMessageType,
    pub input_messages: Vec<u8>
}

impl InputWindowMessage {
    pub fn new(inputs: Vec<u8>) -> InputWindowMessage {
        InputWindowMessage {
            message_type: ToServerMessageType::InputMessage,
            input_messages: inputs
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(C)]
pub struct AwkFrameMessage {
    pub message_type: ToServerMessageType,
    pub frame_number: usize
}

impl AwkFrameMessage {
    pub fn new(frame_number: usize) -> AwkFrameMessage {
        AwkFrameMessage {
            message_type: ToServerMessageType::AwkFrameMessage,
            frame_number
        }
    }
}