
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use bincode::{Decode, Encode};
use std::net::SocketAddr;

use shared_code::*;
use std::collections::HashMap;

pub struct ServerHandshakeManager {
    pub listener: TcpListener,
    pub total_seen_connections: usize,
    pub live_connections: HashMap<usize, TcpStream>
}

impl ServerHandshakeManager {
    pub fn new() -> ServerHandshakeManager {
        let listener = TcpListener::bind("127.0.0.1:34258").unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        ServerHandshakeManager {
            listener,
            total_seen_connections: 0,
            live_connections: HashMap::new()
        }
    }

    pub fn add_new_stream(&mut self, stream: TcpStream) -> usize {
        let player_id = self.total_seen_connections;
        self.total_seen_connections += 1;
        self.live_connections.insert(player_id, stream);
        return player_id;
    }
}

#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(C)]
pub struct PlayerJoinRequest {
    pub stream_id: usize,
    pub socket_address: SocketAddr
}

impl PlayerJoinRequest {
    pub fn new(stream_id: usize, socket_address: SocketAddr) -> PlayerJoinRequest {
        PlayerJoinRequest {
            stream_id,
            socket_address
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(C)]
pub struct PlayerConnectionInfo {
    pub stream_id: usize,
    pub client_id: u8,
    pub port: u16
}
impl PlayerConnectionInfo {
    pub fn new(stream_id: usize, client_id: u8, port: u16) -> PlayerConnectionInfo {
        PlayerConnectionInfo {
            stream_id,
            client_id,
            port
        }
    }
}


pub fn start_handshake_thread() -> (Receiver<PlayerJoinRequest>, Sender<PlayerConnectionInfo>) {

    let (to_game_thread, from_game_thread) : (Sender<PlayerJoinRequest>, Receiver<PlayerJoinRequest>) = channel();
    let (send_to_server, recv_from_client) : (Sender<PlayerConnectionInfo>, Receiver<PlayerConnectionInfo>) = channel();

    let mut server_handshake = ServerHandshakeManager::new();
    thread::spawn(move||{
        loop {
        let mut new_connections = vec![];
        //Consume new connections
        for stream in server_handshake.listener.incoming() {
            match stream {
                Ok(s) => {
                    new_connections.push(s);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    continue;
                }
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }

        for connection in new_connections {
            server_handshake.add_new_stream(connection);
        }
        //TODO: Change this to work off of a struct that wraps the connection
        //That way we can insure that we only escalate and handle handshake in a stepwise manner
        //and also insure that we only handle a single handshake per IP
        //drain exsisting connections for messages
        for (index, connection) in  server_handshake.live_connections.iter_mut() {
            //1500 bytes in the MTU of TCP
            let mut data = [0 as u8; 4]; // using 50 byte buffer
            
            while match connection.read(&mut data) {
                Ok(size) => {
                    if size == 4 {
                        let message = HandshakeMessageType::from_u8(data);
                        match message {
                            HandshakeMessageType::Hello => {
                                //Let the player know that we have heard them
                                let _ = connection.write(&HandshakeMessageType::HelloAwk.to_u8());
                            },
                            HandshakeMessageType::Join => {
                                // We will now need to send a message to the game server and ask it
                                // for a client id and a port
                                let _ = to_game_thread.send(PlayerJoinRequest::new(*index, connection.local_addr().unwrap()));
                            },
                            _ => {
                                println!("Server only handles Hello and Join messages");
                            }
                        }
                    }
                    true
                },
                Err(_) => {

                    false
                }
                
            } {}
        } 
        //send any messages from the server

        let message_iter = recv_from_client.try_iter();
        for message in message_iter {
            let connection = server_handshake.live_connections.get_mut(&message.stream_id);
            match connection {
                Some(stream) => {
                    let _ = stream.write(&HandshakeMessageType::GameSettings(message.client_id, message.port).to_u8());
                },
                None => {
                }
            }
        }
        //TODO: allow for a tcp to be dropped at some point
        //no need to have the connection open if a player is already playing
        }
    });

    return (from_game_thread, send_to_server);
}