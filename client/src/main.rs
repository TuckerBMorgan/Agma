use std::net::UdpSocket;
use shared_code::*;
use bincode::{config, Decode, Encode};
use core::cmp::min;
use storm::color::RGBA8;
use storm::*;
use storm::graphics::{
    shaders::sprite::*, ClearMode, DisplayMode, Texture, TextureSection, Vsync, WindowSettings, DepthTest
};
use storm::asset::Asset;
use storm::math::OrthographicCamera;
use storm::math::PerspectiveCamera;
use storm::cgmath::*;
use storm::event::*;
use storm::graphics::{Buffer, Uniform};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::time::{Duration, Instant};
use std::thread::sleep;
mod rendering;
use rendering::*;
use storm::math::Float;

pub struct Camera {
    /// Transform matix.
    transform: PerspectiveCamera,
    /// Transform uniform.
    uniform: Uniform<ModelUniform>,
    /// Position vector.
    pos: Vector3<f32>,
    /// Unnormalized direction vector.
    dir: Vector3<f32>,
    /// Normalized horizontal xz plane direction vector.
    forward: Vector2<f32>,
    yaw: f32,
    pitch: f32,
    /// Positive is forward.
    pub forward_speed: f32,
    /// Positive is right.
    pub strafe_speed: f32,
    /// Positive is up.
    pub vertical_speed: f32,
    pub multiplier: f32,
}

impl Camera {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> Camera {
        let mut transform = PerspectiveCamera::new(ctx.window_logical_size());
        let uniform = Uniform::new(ctx, &mut transform);
        Camera {
            transform,
            uniform,
            pos: Vector3::zero(),
            dir: Vector3::zero(),
            forward: Vector2::zero(),
            yaw: 0.0,
            pitch: 0.0,
            forward_speed: 0.0,
            strafe_speed: 0.0,
            vertical_speed: 0.0,
            multiplier: 2.0,
        }
    }

    pub fn resize(&mut self, logical_size: Vector2<f32>) {
        self.transform.set_size(logical_size);
        self.uniform.set(&mut self.transform);
    }

    pub fn look(&mut self, cursor_delta: Vector2<f32>) {
        const SENSITIVITY: f32 = 0.12; // Degrees per delta unit.

        self.yaw += cursor_delta.x * SENSITIVITY;
        if self.yaw < 0.0 {
            self.yaw = 360.0 - self.yaw;
        } else if self.yaw > 360.0 {
            self.yaw = self.yaw - 360.0;
        }

        self.pitch += cursor_delta.y * SENSITIVITY;
        if self.pitch < -90.0 {
            self.pitch = -90.0;
        } else if self.pitch > 89.0 {
            self.pitch = 89.0;
        }

        let cos_pitch = self.pitch.cos_deg_fast();
        self.forward = Vector2::new(self.yaw.cos_deg_fast(), self.yaw.sin_deg_fast());
        let x = cos_pitch * self.forward.x;
        let y = self.pitch.sin_deg_fast();
        let z = cos_pitch * self.forward.y;
        self.dir = Vector3::new(x, y, z);
        self.transform.set().direction = self.dir;
        self.uniform.set(&mut self.transform);
    }

    pub fn update(&mut self, time_delta: f32) {
        let forward_speed = time_delta * self.forward_speed * self.multiplier;
        let strafe_speed = time_delta * self.strafe_speed * self.multiplier;
        let vertical_speed = time_delta * self.vertical_speed * self.multiplier;
        self.pos.x += (self.forward.x * forward_speed) + (-self.forward.y * strafe_speed);
        self.pos.z += (self.forward.y * forward_speed) + (self.forward.x * strafe_speed);
        self.pos.y += vertical_speed;
        self.transform.set().eye = self.pos;
        self.uniform.set(&mut self.transform);
    }

    pub fn uniform(&self) -> &Uniform<ModelUniform> {
        &self.uniform
    }
}


pub struct AgmaClientApp {
    encoded_world_states: RingBuffer<(usize, Vec<u8>)>,
    transform: PerspectiveCamera,
    default_texture: Texture,
    transform_uniform: Uniform<ModelUniform>,
    model_shader: ModelShader,
    cube: [ModelVertex;36],
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
            for (index, entity) in game_state.entities.iter().enumerate() {
//                self.transform.set().eye.x = entity.pos.x;
//                self.transform.set().eye.z = entity.pos.z;
            }

            self.particle_buffer.set(&self.cube);
            self.model_shader.draw(&self.camera.uniform(), &self.particle_buffer);
        }
    }
}

impl App for AgmaClientApp {
    fn new(ctx: &mut Context<Self>) -> Self {
        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
        ctx.set_backface_culling(false);
        let mut transform = PerspectiveCamera::new(ctx.window_logical_size());
        let transform_uniform: Uniform<ModelUniform> = Uniform::new(ctx, &mut transform);
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
            model_shader: ModelShader::new(ctx),
            cube: create_cube(),
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
        ctx.set_backface_culling(false);
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
        self.camera.update(delta);
        let encoded: Vec<u8> = bincode::encode_to_vec(&to_server_input_message, config).unwrap();
        self.send_to_server.send(encoded);
        self.render_world();

    }

    fn on_cursor_delta(
        &mut self,
        _ctx: &mut Context<Self>,
        delta: cgmath::Vector2<f32>,
        focused: bool,
    ) {
        self.camera.look(delta);
    }

    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, _is_repeat: bool) {
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            KeyboardButton::W => {
                self.current_input_value |= 1;
            },
            KeyboardButton::S => {
                self.current_input_value |= 2;
            },
            KeyboardButton::A => {
                self.current_input_value |= 4;
            },
            KeyboardButton::D => {
                self.current_input_value |= 8;
            },
            _ => {
            }
        }
    }

    fn on_key_released(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton) {
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            KeyboardButton::W => {
                self.current_input_value &= !(1);
            },
            KeyboardButton::S => {
                self.current_input_value &= !(2);
            },
            KeyboardButton::A => {
                self.current_input_value &= !(4);
            },
            KeyboardButton::D => {
                self.current_input_value &= !(8);
            },
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