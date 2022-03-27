use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use bincode::*;
use shared_code::*;
use std::time::Duration;
use std::thread::sleep;
use std::thread;
use std::net::UdpSocket;

pub fn start_player_thread(server_address: String) -> (Receiver<UpdateWorldMessage>, Sender<Vec<u8>>) {
    let (send_to_client, recv_from_server) : (Sender<UpdateWorldMessage>, Receiver<UpdateWorldMessage>) = channel();
    let (send_to_server, recv_from_client) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    thread::spawn(move||{
        let socket =  UdpSocket::bind(server_address).unwrap();
        let mut buf = [0; 1000];
        loop {
            //TODO: drain this of all updates from the server
            let (amt, _src) = socket.recv_from(&mut buf).unwrap();
            if amt == 0 {
                return;
            }
            let buf = &mut buf[..amt];
            let config = config::standard();
            let (foo, _len): (UpdateWorldMessage, usize) = bincode::serde::decode_from_slice(&buf[..], config).unwrap();

            let _ = send_to_client.send(foo);
            let to_server = recv_from_client.try_iter();

            for message in to_server {
                let _ = socket.send_to(&message, "127.0.0.1:34257");
            }
            sleep(Duration::from_millis(16));
        }
    });

    (recv_from_server, send_to_server)
}