use shared_code::*;
use std::net::{ UdpSocket};
use std::time::Duration;
use std::thread::sleep;
use bincode::config;
use cgmath::*;

pub fn produce_state_difference(old_state: &Vec<u8>, new_state: &Vec<u8>) -> Vec<u8> {
    let iter = old_state.iter().zip(new_state.iter());
    let mut result = Vec::with_capacity(new_state.len());
    for (a, b) in iter {
        result.push(a ^ b);
    }
    //If we have some extra information to add, add it to the send vector
    if result.len() < new_state.len() {
        let dif = new_state.len() - result.len();
        let result_first_len = result.len();
        //We will keep this as we need it as the offset into result

        for i in 0..dif {
            let index = result_first_len + i;
            result.push(new_state[index]);
        }
    }
    return result;
}

pub struct PlayerConnection {
    pub previous_game_state: RingBuffer<(usize, Vec<u8>)>,
    pub last_awked_game_state: usize,
    pub udp_socket: UdpSocket,
    pub inputs: Vec<u8>
}

impl PlayerConnection {
    pub fn new() -> PlayerConnection {
        let udp_socket = UdpSocket::bind("127.0.0.1:34257").unwrap();
        let _ = udp_socket.set_nonblocking(true);
        PlayerConnection {
            previous_game_state: RingBuffer::new(),
            last_awked_game_state: 0,
            udp_socket,
            inputs: vec![]
        }
    }

    pub fn update_player_with_new_game_state(&mut self, buffer: Vec<u8>, frame_number: usize) {
        let config = config::standard();
        if self.previous_game_state.next_open_slot != 0 && self.last_awked_game_state != 0 {
            let mut index = 0;
            let mut has_previous_frame = false;
            for (i, state) in self.previous_game_state.storage.iter().enumerate() {
                if state.0 == self.last_awked_game_state as usize {
                    //generate a new
                    index = i;
                    has_previous_frame = true;
                    break;
                }
            }
            if has_previous_frame {
                let last_state = &self.previous_game_state.storage[index];
                let mut to_player_message = UpdateWorldMessage::new(frame_number, self.last_awked_game_state, produce_state_difference(&last_state.1, &buffer));
                to_player_message.message_type = ToPlayerMessageType::UpdateWorld;
                let encoded: Vec<u8> = bincode::serde::encode_to_vec(&to_player_message, config).unwrap();
                let _ = self.udp_socket.send_to(&encoded, "127.0.0.1:34255");
                self.previous_game_state.add_new_data((frame_number, buffer));
            }
            else {
                let to_player_message = UpdateWorldMessage::new(frame_number, self.last_awked_game_state, buffer.clone());
                let encoded: Vec<u8> = bincode::serde::encode_to_vec(&to_player_message, config).unwrap();
                let _ = self.udp_socket.send_to(&encoded, "127.0.0.1:34255");
                self.previous_game_state.add_new_data((frame_number, buffer));   
            }
        }
        else {
            let to_player_message = UpdateWorldMessage::new(frame_number, self.last_awked_game_state, buffer.clone());
            let encoded: Vec<u8> = bincode::serde::encode_to_vec(&to_player_message, config).unwrap();
            let _ = self.udp_socket.send_to(&encoded, "127.0.0.1:34255");
            self.previous_game_state.add_new_data((frame_number, buffer));
        }
    }

    pub fn check_on_player(&mut self) {
        let config = config::standard();
        let mut buf = [0; 1000];
        loop {
            //We want to drain the input buffer for each player
            match self.udp_socket.recv_from(&mut buf) {
                Ok((amt, _src)) => {
                    if amt == 0 {
                        return;
                    }

                    let buf = &mut buf[..amt];
                    let message_type = PlayerToServerMessage::from_u8(buf[0]);
                    match message_type {
                        PlayerToServerMessage::AwkFrameMessage => {
                            let (msg, _len) : (AwkFrameMessage, usize) = bincode::serde::decode_from_slice(&buf[..], config).unwrap();
                            if msg.frame_number > self.last_awked_game_state {
                                self.last_awked_game_state = msg.frame_number;
                            }
                        },
                        PlayerToServerMessage::KeyboardActionMessage => {
                            let (msg, _len) : (KeyboardActionMessage, usize) = bincode::serde::decode_from_slice(&buf[..], config).unwrap();
                            if msg.input_messages.len() <= 16 {
                                self.inputs = msg.input_messages;
                            }
                        },
                        PlayerToServerMessage::MouseAction => {
                            println!("Ignoring these messages for now");
                        }
                        _ => {

                        }
                    }
                }
                Err(_e) => {return;/*println!("failed {:?}", e)*/}
            }
        }
    }
}

fn main() {
    let mut player_connection = PlayerConnection::new();
    let mut w = World::default();
    for i in 0..1 {
        let entity_id = w.spawn_entity();
        w.add_component(entity_id, TransformComponent::new(Matrix4::from_translation(Vector3::new(0.0f32, 0.0, 0.0))));
    }
    let config = config::standard();
    loop {
        player_connection.check_on_player();
        if player_connection.inputs.len() > 0 {
            let input = player_connection.inputs.remove(0);
            w.input = input;
        }
        w.tick();
        let encoded: Vec<u8> = bincode::serde::encode_to_vec(&w, config).unwrap();
        player_connection.update_player_with_new_game_state(encoded, w.frame_number);        
        w.post_tick();
        sleep(Duration::from_millis(16));
    }
}
