use shared_code::*;
use std::net::{ UdpSocket};
use std::time::{Duration, Instant};
use std::thread::sleep;

mod math;

pub fn produce_state_difference(old_state: &Vec<u8>, new_state: &Vec<u8>) -> Vec<u8> {
    let mut iter = old_state.iter().zip(new_state.iter());
    let mut result = Vec::with_capacity(new_state.len());
    for (a, b) in iter {
        result.push(a ^ b);
    }
    //If we have some extra information to add, add it to the send vector
    if result.len() < new_state.len() {
        let dif = new_state.len() - result.len();
        //We will keep this as we need it as the offset into result
        let length_of_new_state = new_state.len();
        for i in 0..dif {
            result.push(new_state[result.len() + dif]);
        }
    }
    return result;
}

pub enum PlayerMessageType {
    UpdateWorld,
    StateWorld
}

pub struct PlayerConnection {
    pub previous_game_state: HashMap<Vec<u8>>,
    pub last_awked_game_state: usize,
    pub udp_socket: UdpSocket
}

impl PlayerConnection {
    pub fn new() -> PlayerConnection {
        PlayerConnection {
            previous_game_state: vec![],
            last_awked_game_state: 0,
            udp_socket: UdpSocket::bind("127.0.0.1:34255")
        }
    }

    pub fn update_player_with_new_game_state(&mut self, buffer: Vec<u8>, frame_number: usize) {
        let last_state = self.previous_game_state[&self.last_awked_game_state];
        let mut to_player_message = UpdateWorldMessage::new(frame_number, self.last_awked_game_state, produce_state_difference(last_state, &buffer));
        if self.last_awked_game_state != 0 {
            to_player_message.message_type = PlayerMessageType::UpdateWorld;
        }
        self.udp_socket.send(to_player_message);
        self.previous_game_state.insert(frame_number, buffer);
    }
}

pub struct UpdateWorldMessage {
    pub message_type: PlayerMessageType,
    pub current_frame_number: usize,
    pub delta_frame_number: usize,
    pub data: Vec<u8>
}

impl UpdateWorldMessage {
    pub fn new(current_frame_number: usize, delta_frame_number: usize, data: Vec<u8>) -> UpdateWorldMessage {
        UpdateWorldMessage {
            message_type: PlayerMessageType::StateWorld,
            current_frame_number,
            delta_frame_number,
            data
        }
    }
}


fn main() {

    let mut player_connection = PlayerConnection::new();
    let mut w = World::default();
    let mut test_entities = vec![];
    for i in 0..3 {
        let mut entities = Entity::default();
        entities.pos = Vec3::new(0.0, 0.0, i as f32);
        test_entities.push(entities);
    }
    w.entities = test_entities;
    loop {  
        player_connection.update_player_with_new_game_state(bincode::encode(w), w.frame_number);        
        sleep(Duration::from_millis(16));
    }
}
