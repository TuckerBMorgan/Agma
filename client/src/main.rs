use shared_code::*;
use core::cmp::min;
use storm::color::RGBA8;
use storm::*;
use storm::graphics::{ClearMode, DisplayMode, Vsync,Texture,TextureFiltering, WindowSettings, DepthTest};
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

use log::{info, trace, warn};
use log::LevelFilter;
mod rendering;
use rendering::*;

mod networking;
use networking::*;


static TEXTURE_A: &[u8] = include_bytes!("resources/images/floor.png");


pub struct AgmaClientApp {
    encoded_world_states: RingBuffer<(usize, Vec<u8>)>,
    
    particle_buffer: Buffer<ModelVertex>,
    cube: Vec<ModelVertex>,
    model_shader: ModelShader,

    floor_buffer: Buffer<TexturedVertex>,    
    floor: Vec<TexturedVertex>,
    texture_shader: TextureShader,
    floor_texture: Texture,

    latest_game_state: Option<World>,
    recv_from_server: Receiver<UpdateWorldMessage>,
    send_to_server: Sender<Vec<u8>>,
    previous_inputs: Vec<u8>,
    previous_mouse_inputs: Vec<MouseState>,
    current_input_value: u8,
    current_mouse_input_value: u8,
    camera: Camera
}

impl AgmaClientApp {
    fn handle_message_update(&mut self, mut message: UpdateWorldMessage) {
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
                    let mut encoded: Vec<u8> = serde_json::to_vec(&awk_frame_message).unwrap();
                    encoded.insert(0, awk_frame_message.message_type.to_u8());
                    let _ = self.send_to_server.send(encoded);
                    self.encoded_world_states.add_new_data((message.current_frame_number, message.data.clone()));

                    if self.latest_game_state.is_none() || self.latest_game_state.as_ref().unwrap().frame_number < message.current_frame_number {

                        let world : World = serde_json::from_slice(&message.data).unwrap();

                        self.latest_game_state = Some(world);
                    }
                }
                else {
                    //If we could not find a frame to build the delta off of, just ignore it
                    println!("NOTNAKJSDLAKSJDALSKDJ");
                }
            },
            ToPlayerMessageType::StateWorld => {
                let world : World = serde_json::from_slice(&message.data[..]).unwrap();
                self.encoded_world_states.add_new_data((message.current_frame_number, message.data));
                let awk_frame_message = AwkFrameMessage::new(message.current_frame_number);
                let mut encoded: Vec<u8> = serde_json::to_vec(&awk_frame_message).unwrap();
                encoded.insert(0, awk_frame_message.message_type.to_u8());
                let _ = self.send_to_server.send(encoded);
                self.latest_game_state = Some(world);
            }
        }
    }

    fn render_world(&mut self) {
        self.floor_buffer.set(&self.floor.as_slice());
        self.texture_shader.draw(&self.camera.model_view_projection_uniform(&Matrix4::from_scale(100.0)), &self.floor_texture, &self.floor_buffer);

        if self.latest_game_state.is_some() {
            let game_state = self.latest_game_state.as_ref().unwrap();
            for transform in game_state.entities.iter() {
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
            floor_buffer: Buffer::new(ctx),    
            floor: create_plane().to_vec(),
            texture_shader: TextureShader::new(ctx),
            floor_texture: Texture::from_png(ctx, TEXTURE_A, TextureFiltering::none()),
            latest_game_state: None,
            recv_from_server,
            send_to_server,
            previous_inputs: vec![],
            previous_mouse_inputs: vec![],
            current_input_value: 0,
            current_mouse_input_value: 0,
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
                world.client_tick();
            },
            None => {

            }
        }

        if self.previous_inputs.len() >= 16 {
            self.previous_inputs.remove(0);
        }
        self.current_input_value = 0;
        self.previous_inputs.push(self.current_input_value);
        let to_server_input_message = KeyboardActionMessage::new(self.previous_inputs.clone());

        if self.previous_mouse_inputs.len() >= 16 {
            self.previous_mouse_inputs.remove(0);
        }
        let mut encoded: Vec<u8> = serde_json::to_vec(&to_server_input_message).unwrap();
        encoded.insert(0, to_server_input_message.message_type.to_u8());
        let _ = self.send_to_server.send(encoded);

        if self.latest_game_state.is_some() {
            self.previous_mouse_inputs.push(MouseState::new(self.current_mouse_input_value != 0, self.latest_game_state.as_ref().unwrap().entities[0].position() + Vector3::new(1.0, 0.0, 0.0)));
            let to_server_mouse_input_message = MouseActionMessage::new(self.previous_mouse_inputs.clone());
            self.latest_game_state.as_mut().unwrap().click_inputs = self.previous_mouse_inputs.iter().map(|x|WorldMouseState::new(x)).collect();
            let mut encoded: Vec<u8> = serde_json::to_vec(&to_server_mouse_input_message).unwrap();
            encoded.insert(0, to_server_mouse_input_message.message_type.to_u8());
            let _ = self.send_to_server.send(encoded);
        }

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

    fn on_cursor_pressed(
        &mut self,
        _ctx: &mut Context<Self>,
        button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        _normalized_pos: cgmath::Vector2<f32>,
    ) {
        match button {
            CursorButton::Right => {
                self.current_mouse_input_value |= 1;
            },
            CursorButton::Left => {
                self.current_mouse_input_value |= 2;
            },
            _ => {

            }
        }
    }

    fn on_cursor_released(
        &mut self,
        _ctx: &mut Context<Self>,
        button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        _normalized_pos: cgmath::Vector2<f32>,
    ) {
        match button {
            CursorButton::Right => {
                self.current_mouse_input_value &= !(1);
            },
            CursorButton::Left => {
                self.current_mouse_input_value &= !(2);
            },
            _ => {

            }
        }
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
    let _ = simple_logging::log_to_file("test.log", LevelFilter::Info);
    info!("Razor located");
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