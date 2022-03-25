use std::net::UdpSocket;
use shared_code::*;
use bincode::{config, Decode, Encode};
use core::cmp::min;
use storm::color::RGBA8;
use storm::*;
use storm::graphics::{
    shaders::sprite::*, ClearMode, DisplayMode, Texture, TextureSection, Vsync, WindowSettings,
};
use storm::asset::Asset;
use storm::math::OrthographicCamera;
use storm::cgmath::*;
use storm::event::*;
use storm::graphics::{Buffer, Uniform};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::time::{Duration, Instant};
use std::thread::sleep;

struct AgmaClientApp {
    encoded_world_states: RingBuffer<(usize, Vec<u8>)>,
    transform: OrthographicCamera,
    default_texture: Texture,
    transform_uniform: Uniform<SpriteUniform>,
    sprite_shader: SpriteShader,
    sprites: Vec<Sprite>,
    particle_buffer: Buffer<Sprite>,
    latest_game_state: Option<World>,
    recv_from_server: Receiver<UpdateWorldMessage>,
    send_to_server: Sender<Vec<u8>>,
    previous_inputs: RingBuffer<u8>,
    current_input_value: u8

}

impl AgmaClientApp {
    fn handle_message_update(&mut self, mut message: UpdateWorldMessage) {
        let config = config::standard();
        match message.message_type {
            ToPlayerMessageType::UpdateWorld => {
                let mut index = 0;

                let mut has_previous_frame = false;
                for (i, state) in self.encoded_world_states.storage.iter().enumerate() {
                    if state.0 == message.delta_frame_number as usize {
                        //generate a new
                        index = i;
                        has_previous_frame = true;
                        break;
                    }
                }
                if has_previous_frame == true {
                    let (frame_number, data) = &self.encoded_world_states.storage[index];
                    let loop_counter = min(data.len(), message.data.len());
                    for i in 0..loop_counter {
                        message.data[i] = message.data[i] ^ data[i];
                    }

                    let awk_frame_message = AwkFrameMessage::new(message.current_frame_number);
                    let config = config::standard();
                    let encoded: Vec<u8> = bincode::encode_to_vec(&awk_frame_message, config).unwrap();
                    self.send_to_server.send(encoded);
                    self.encoded_world_states.add_new_data((message.current_frame_number, message.data.clone()));


                    if self.latest_game_state.is_none() || self.latest_game_state.as_ref().unwrap().frame_number < message.current_frame_number {
                        self.latest_game_state = Some(bincode::decode_from_slice(&message.data, config).unwrap().0);

                    }
                }
                else {
                    //If we could not find a frame to build the delta off of, just ignore it
                }
            },
            ToPlayerMessageType::StateWorld => {
                let (world, len) : (World, usize) = bincode::decode_from_slice(&message.data[..], config).unwrap();
                self.encoded_world_states.add_new_data((message.current_frame_number, message.data));
                let config = config::standard();
                let awk_frame_message = AwkFrameMessage::new(message.current_frame_number);
                let encoded: Vec<u8> = bincode::encode_to_vec(&awk_frame_message, config).unwrap();
                self.send_to_server.send(encoded);
                self.latest_game_state = Some(world);
            }
        }
    }

    fn render_world(&mut self) {
        if self.latest_game_state.is_some() {
            let game_state = self.latest_game_state.as_ref().unwrap();
            if game_state.entities.len() > self.sprites.len() {
                let missing_amount = game_state.entities.len() - self.sprites.len();
                for i in 0..missing_amount {
                    let new_sprite = Sprite::new(Vector3::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY), Vector2::new(1.0, 1.0), TextureSection::full(), RGBA8::WHITE, 0.0);
                    self.sprites.push(Sprite::default());
                }
            }
            for (index, entity) in game_state.entities.iter().enumerate() {
                self.sprites[index].pos = Vector3::new(entity.pos.x, entity.pos.z, 0.0);
            }
            self.particle_buffer.set(&self.sprites);
            self.sprite_shader.draw(&self.transform_uniform, &self.default_texture, &[&self.particle_buffer]);
        }
    }
}

impl App for AgmaClientApp {
    fn new(ctx: &mut Context<Self>) -> Self {

        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
        let mut transform = OrthographicCamera::new(ctx.window_logical_size());
        let transform_uniform: Uniform<SpriteUniform> = Uniform::new(ctx, &mut transform);
        let (send_to_client, recv_from_server) : (Sender<UpdateWorldMessage>, Receiver<UpdateWorldMessage>) = channel();
        let (send_to_server, recv_from_client) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
        thread::spawn(move||{
            let socket =  UdpSocket::bind("127.0.0.1:34255").unwrap();
            let mut buf = [0; 1000];
            loop {
                //TODO: drain this of all updates from the server
                let (amt, src) = socket.recv_from(&mut buf).unwrap();
                if amt == 0 {
                    return;
                }
                let buf = &mut buf[..amt];
                let config = config::standard();
                let (mut foo, len): (UpdateWorldMessage, usize) = bincode::decode_from_slice(&buf[..], config).unwrap();

                send_to_client.send(foo);
                let to_server = recv_from_client.try_iter();

                for message in to_server {

                    socket.send_to(&message, "127.0.0.1:34257");
                }
                sleep(Duration::from_millis(16));
            }
        });

        AgmaClientApp {
            encoded_world_states: RingBuffer::new(),
            transform,
            default_texture: ctx.default_texture(),
            transform_uniform,
            sprite_shader: SpriteShader::new(ctx),
            sprites: vec![],
            particle_buffer: Buffer::new(ctx),
            latest_game_state: None,
            recv_from_server,
            send_to_server,
            previous_inputs: RingBuffer::new(),
            current_input_value: 0 
        }
    }

    fn on_close_requested(&mut self, ctx: &mut Context<Self>) {
        ctx.request_stop();
    }

    fn on_update(&mut self, ctx: &mut Context<Self>, _delta: f32) {
        self.current_input_value = 0;
        ctx.clear(ClearMode::color_depth(RGBA8::BLACK));
        // the message, it will be cut off.

        let from_server_message = self.recv_from_server.try_recv();

        match from_server_message {
            Ok(message) => {
                self.handle_message_update(message);
            },
            Err(_) => {}
        }

        let config = config::standard();
        self.previous_inputs.add_new_data(self.current_input_value);
        let to_server_input_message = InputWindowMessage::new(self.previous_inputs.storage.clone());
        let encoded: Vec<u8> = bincode::encode_to_vec(&to_server_input_message, config).unwrap();
        self.send_to_server.send(encoded);
        self.render_world();
    }

    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, _is_repeat: bool) {
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            KeyboardButton::W => {
                self.current_input_value &= 1;                
            },
            KeyboardButton::A => {
                self.current_input_value &= 2;
            },
            KeyboardButton::S => {
                self.current_input_value &= 4;
            },
            KeyboardButton::D => {
                self.current_input_value &= 8;
            },
            _ => {

            }
        }
    }

    fn on_key_released(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton) {
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            _ => {

            }
        }
    }
}

fn main() {
    start::<AgmaClientApp>(
        WindowSettings {
            title: String::from("Agma"),
            display_mode: DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            },
        vsync: Vsync::Disabled,
        }
    );
}