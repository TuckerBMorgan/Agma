use crate::*;
use std::net::{UdpSocket, SocketAddr};


pub struct ServerSocket {
    pub socket: UdpSocket
}

impl ServerSocket {
    pub fn new() -> ServerSocket {
        let udp_socket = UdpSocket::bind("127.0.0.1:34257").unwrap();
        let _ = udp_socket.set_nonblocking(true);
        ServerSocket {
            socket: udp_socket
        }
    }
    
    /// Main function for looking into the state of the socket for the player
    /// this will update what the player has send us
    /// best to be called as often as possible to help keep the UDP buffer
    /// as small as possible
    pub fn check_on_players(&mut self, player_connections: &mut HashMap<SocketAddr, PlayerConnection>) {
        let mut buf = [0; 65507];
        let config = config::standard();
        loop {
            //We want to drain the input buffer for each player
            match self.socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    if amt == 0 {
                        return;
                    }
                    let mut player_connection = player_connections.get_mut(&src).unwrap();
                    let buf = &mut buf[..amt];
                    let buf = bitfield_rle::decode(&buf[..]).unwrap();
                    let message_type = PlayerToServerMessage::from_u8(buf[0]);
                    match message_type {
                        PlayerToServerMessage::AwkFrameMessage => {
                            let (msg, _len) : (AwkFrameMessage, usize) = bincode::decode_from_slice(&buf[1..], config).unwrap();
                            if msg.frame_number > player_connection.last_awked_game_state {
                                player_connection.last_awked_game_state = msg.frame_number;
                            }
                        },
                        PlayerToServerMessage::KeyboardAction => {
                            let (msg, _len) : (KeyboardActionMessage, usize) = bincode::decode_from_slice(&buf[1..], config).unwrap();
                            if msg.input_messages.len() <= 16 {
                                player_connection.inputs = msg.input_messages;
                            }
                        },
                        PlayerToServerMessage::MouseAction => {
                            let (msg, _len) : (MouseActionMessage, usize) = bincode::decode_from_slice(&buf[1..], config).unwrap();
                            if msg.destinations.len() <= 16 {
                                player_connection.desired_inputs = msg.destinations;
                            }
                        }
                    }
                }
                Err(_e) => {return;/*println!("failed {:?}", e)*/}
            }
        }
    }
    
}


/// A component used to tie a character to a UDP socket
/// so it can be used to drive that characters movement
#[derive(Encode, Decode, PartialEq, Debug, Copy, Clone)]
pub struct PlayerConnectionComponent {
    /// The player index, this is a 1-1 mapping to an array of PlayerConnections
    pub player_index: usize,
}

impl PlayerConnectionComponent {
    pub fn new(player_index: usize) -> PlayerConnectionComponent {
        PlayerConnectionComponent {
            player_index
        }
    }
}

/// A helper function that will produce a diff between two sets of u8s, such that
/// later the diff can be used to recreate the new_state
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

/// A wrapper struct around a UDP socket, and a collection of previous game states
/// this is the main interface with a player for the server
pub struct PlayerConnection {
    /// a ring buffer of previous game states in byte array, used to create diffs
    pub previous_game_state: RingBuffer<(usize, Vec<u8>)>,
    /// the number of the last frame this player has told us they have heard
    pub last_awked_game_state: usize,
    /// the inputs we have heard, but have yet to move over to the main thread
    pub inputs: Vec<u8>,
    pub desired_inputs: Vec<MouseState>
}

impl PlayerConnection {
    pub fn new(_ip: String) -> PlayerConnection {
        PlayerConnection {
            previous_game_state: RingBuffer::new(),
            last_awked_game_state: 0,
            inputs: vec![],
            desired_inputs: vec![]
        }
    }

    /// Given a encoded game state, and which frame it is based on send the player
    /// either the whole buffer(if the player does not have that frame)
    /// or a delta state from a frame we know the player has, so the player can rebuild the state later
    pub fn update_player_with_new_game_state(&mut self, buffer: Vec<u8>, frame_number: usize, udp_socket: &mut UdpSocket) {
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
                let encoded =  bincode::encode_to_vec(to_player_message, config).unwrap();
                let encoded = bitfield_rle::encode(encoded);
                let _ = udp_socket.send_to(&encoded, "127.0.0.1:34255");
                self.previous_game_state.add_new_data((frame_number, buffer));
            }
            else {
                let to_player_message = UpdateWorldMessage::new(frame_number, self.last_awked_game_state, buffer.clone());
                let encoded =  bincode::encode_to_vec(to_player_message, config).unwrap();
                let encoded = bitfield_rle::encode(encoded);
                let _ = udp_socket.send_to(&encoded, "127.0.0.1:34255");
                self.previous_game_state.add_new_data((frame_number, buffer));   
            }
        }
        else {
            let to_player_message = UpdateWorldMessage::new(frame_number, self.last_awked_game_state, buffer.clone());
            let encoded =  bincode::encode_to_vec(to_player_message, config).unwrap();
            let encoded = bitfield_rle::encode(encoded);
            let _ = udp_socket.send_to(&encoded, "127.0.0.1:34255");
            self.previous_game_state.add_new_data((frame_number, buffer));
        }
    }


}