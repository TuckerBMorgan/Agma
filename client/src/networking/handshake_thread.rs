use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::net::{TcpStream};
use std::io::{Read, Write};
use shared_code::*;
use std::net::SocketAddr;

pub struct ServerConnectionInfo {
    pub client_id: u8,
    pub port: u16
}

impl ServerConnectionInfo {
    pub fn new(client_id: u8, port: u16) -> ServerConnectionInfo {
        ServerConnectionInfo {
            client_id,
            port
        }
    }
}

pub fn preform_handshake(server_ip: SocketAddr) -> Receiver<ServerConnectionInfo> {
    let (send_to_client, recv_from_server) : (Sender<ServerConnectionInfo>, Receiver<ServerConnectionInfo>) = channel();
    thread::spawn(move||{
        let mut stream = TcpStream::connect(server_ip).unwrap();
        println!("Successfully connected to server {:?}", server_ip);
        let msg = HandshakeMessageType::Hello;
        stream.write(&msg.to_u8()).unwrap();
        println!("Sent Hello, awaiting reply...");
        loop {
            let mut data = [0 as u8; 50]; // using 6 byte buffer
            match stream.read(&mut data) {
                Ok(_) => {
                    let mut test_data = [0; 4];
                    test_data[0] = data[0];
                    test_data[1] = data[1];
                    test_data[2] = data[2];
                    test_data[3] = data[3];
                    let message = HandshakeMessageType::from_u8(test_data);
                    match message {
                        HandshakeMessageType::HelloAwk => {
                            let join_message = HandshakeMessageType::Join;
                            let _ = stream.write(&join_message.to_u8());
                            println!("Server responded, asking to join game");
                        },
                        HandshakeMessageType::GameSettings(client_id, port) => {
                            let server_connection_info = ServerConnectionInfo::new(client_id, port);
                            let _ = send_to_client.send(server_connection_info);
                            println!("Joining game with id {:?}, on port {:?}", client_id, port);
                        },
                        _ => {
                            println!("The client should only be getting HelloAwk, and GameSettings message");
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }


        println!("Terminated.");
    });

    return recv_from_server;
}