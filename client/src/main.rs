use shared_code::*;
use core::cmp::min;
use storm::color::RGBA8;
use storm::*;
use storm::graphics::{ClearMode, DisplayMode, Vsync, WindowSettings, DepthTest};
use std::ops::{Mul};
use storm::math::PerspectiveCamera;
use storm::cgmath::*;
use storm::event::*;
use storm::graphics::{Buffer, Uniform};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use std::thread::sleep;
use storm::math::Float;
use bincode::*;

mod rendering;
use rendering::*;

mod networking;
use networking::*;


pub struct AgmaClientApp {
    encoded_world_states: RingBuffer<(usize, Vec<u8>)>,
    model_shader: ModelShader,
    cube: Vec<ModelVertex>,
    particle_buffer: Buffer<ModelVertex>,
    latest_game_state: Option<World>,
    recv_from_server: Receiver<UpdateWorldMessage>,
    send_to_server: Sender<Vec<u8>>,
    previous_inputs: Vec<u8>,
    current_input_value: u8,
    camera: Camera
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
                    let (_frame_number, data) = &self.encoded_world_states.storage[index];
                    let loop_counter = min(data.len(), message.data.len());
                    for i in 0..loop_counter {
                        message.data[i] = message.data[i] ^ data[i];
                    }

                    let awk_frame_message = AwkFrameMessage::new(message.current_frame_number);
                    let config = config::standard();
                    let encoded: Vec<u8> = bincode::serde::encode_to_vec(&awk_frame_message, config).unwrap();
                    let _ = self.send_to_server.send(encoded);
                    self.encoded_world_states.add_new_data((message.current_frame_number, message.data.clone()));


                    if self.latest_game_state.is_none() || self.latest_game_state.as_ref().unwrap().frame_number < message.current_frame_number {
                        self.latest_game_state = Some(bincode::serde::decode_from_slice(&message.data, config).unwrap().0);
                    }
                }
                else {
                    //If we could not find a frame to build the delta off of, just ignore it
                }
            },
            ToPlayerMessageType::StateWorld => {
                let (world, _len) : (World, usize) = bincode::serde::decode_from_slice(&message.data[..], config).unwrap();
                self.encoded_world_states.add_new_data((message.current_frame_number, message.data));
                let config = config::standard();
                let awk_frame_message = AwkFrameMessage::new(message.current_frame_number);
                let encoded: Vec<u8> = bincode::serde::encode_to_vec(&awk_frame_message, config).unwrap();
                let _ = self.send_to_server.send(encoded);
                self.latest_game_state = Some(world);
            }
        }
    }

    fn render_world(&mut self) {
        if self.latest_game_state.is_some() {
            let game_state = self.latest_game_state.as_ref().unwrap();
            for transform in game_state.transforms.iter() {
                self.particle_buffer.set(&self.cube.as_slice());
                self.model_shader.draw(&self.camera.model_view_projection_uniform(&transform.transform), &self.particle_buffer);
            }
        }    
    }
}

impl App for AgmaClientApp {
    fn new(ctx: &mut Context<Self>) -> Self {
        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
        let (recv_from_server, send_to_server) = start_player_thread(String::from("127.0.0.1:34255"));

        AgmaClientApp {
            encoded_world_states: RingBuffer::new(),
            model_shader: ModelShader::new(ctx),
            cube: create_cube().to_vec(),
            particle_buffer: Buffer::new(ctx),
            latest_game_state: None,
            recv_from_server,
            send_to_server,
            previous_inputs: vec![],
            current_input_value: 0,
            camera: Camera::new(ctx)
        }
    }

    fn on_close_requested(&mut self, ctx: &mut Context<Self>) {
        ctx.request_stop();
    }

    fn on_update(&mut self, ctx: &mut Context<Self>, delta: f32) {
        ctx.clear(ClearMode::new().with_color(RGBA8::BLUE).with_depth(0.0, DepthTest::Greater));
        // the message, it will be cut off.

        let from_server_message = self.recv_from_server.try_recv();

        match from_server_message {
            Ok(message) => {
                self.handle_message_update(message);
            },
            Err(_) => {}
        }

        match &mut self.latest_game_state {
            Some(world) => {
                world.input = self.current_input_value;
                world.tick();
            },
            None => {

            }
        }

        let config = config::standard();
        if self.previous_inputs.len() >= 16 {
            self.previous_inputs.remove(0);
        }

        self.previous_inputs.push(self.current_input_value);
        let to_server_input_message = InputWindowMessage::new(self.previous_inputs.clone());

        let encoded: Vec<u8> = bincode::serde::encode_to_vec(&to_server_input_message, config).unwrap();
        let _ = self.send_to_server.send(encoded);

        self.camera.update(delta);
        self.render_world();
    }

    fn on_cursor_delta(
        &mut self,
        _ctx: &mut Context<Self>,
        delta: cgmath::Vector2<f32>,
        _focused: bool,
    ) {
        self.camera.look(delta);
    }

    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, is_repeat: bool) {
        if is_repeat {
            return;
        }
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            KeyboardButton::W => {
                self.camera.forward_speed += 1.0;
                self.current_input_value |= 1;
            }
            KeyboardButton::S => {
                self.camera.forward_speed -= 1.0;
                self.current_input_value |= 2;
            }
            KeyboardButton::A => {
                self.camera.strafe_speed -= 1.0;
                self.current_input_value |= 4;
            }
            KeyboardButton::D => {
                self.camera.strafe_speed += 1.0;
                self.current_input_value |= 8;
            }
            KeyboardButton::Space => {
                self.camera.vertical_speed += 1.0;
            }
            KeyboardButton::LShift => {
                self.camera.vertical_speed -= 1.0;
            }
            KeyboardButton::LControl => {
                self.camera.multiplier += 4.0;
            }
            _ => {}
        }
    }

    fn on_key_released(&mut self, _ctx: &mut Context<Self>, key: event::KeyboardButton) {
        match key {
            KeyboardButton::W => {
                self.camera.forward_speed -= 1.0;
                self.current_input_value &= !(1);
            }
            KeyboardButton::S => {
                self.camera.forward_speed += 1.0;
                self.current_input_value &= !(2);
            }
            KeyboardButton::A => {
                self.camera.strafe_speed += 1.0;
                self.current_input_value &= !(4);
            }
            KeyboardButton::D => {
                self.camera.strafe_speed -= 1.0;
                self.current_input_value &= !(8);
            }
            KeyboardButton::Space => {
                self.camera.vertical_speed -= 1.0;
            }
            KeyboardButton::LShift => {
                self.camera.vertical_speed += 1.0;
            }
            KeyboardButton::LControl => {
                self.camera.multiplier -= 4.0;
            }
            _ => {}
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