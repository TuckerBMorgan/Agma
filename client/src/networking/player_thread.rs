use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use shared_code::*;
use std::time::Duration;
use std::thread::sleep;
use std::thread;
use std::net::UdpSocket;
use bincode::*;

pub fn start_player_thread(server_address: String) -> (Receiver<UpdateWorldMessage>, Sender<Vec<u8>>) {
    let (send_to_client, recv_from_server) : (Sender<UpdateWorldMessage>, Receiver<UpdateWorldMessage>) = channel();
    let (send_to_server, recv_from_client) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    let config = config::standard();
    thread::spawn(move||{
        let socket =  UdpSocket::bind(server_address).unwrap();
        //We want to have this be non blocking, so we can alternate between
        //looking for messages from the client to sent up
        //and looking for message down from the server
        let _ = socket.set_nonblocking(true);
        let mut buf = [0; 65507];
        loop {
            loop {
                //Loop so we drain the messages from the server, making sure we have the most up to date
                //world states we can
                match socket.recv_from(&mut buf) {
                    Ok((amt, _src)) => {
                        if amt == 0 {
                            break;
                        }
                        let buf = &mut buf[..amt];
                        let decode = bitfield_rle::decode(&buf[..]).unwrap();
                        let (foo, _len): (UpdateWorldMessage, usize) = bincode::decode_from_slice(&decode, config).unwrap();
                        let _ = send_to_client.send(foo);
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
            let to_server = recv_from_client.try_iter();
            for message in to_server {
                let message = bitfield_rle::encode(message);
                let _ = socket.send_to(&message, "127.0.0.1:34257");
            }
            sleep(Duration::from_millis(16));
        }
    });

    (recv_from_server, send_to_server)
}