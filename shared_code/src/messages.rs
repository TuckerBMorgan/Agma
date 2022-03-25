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