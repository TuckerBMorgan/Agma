use crate::*;
use std::sync::mpsc::{Sender, Receiver};
use crate::networking::*;
use crate::rendering::*;
use bincode::{config, Decode, Encode};
use storm::cgmath::*;

pub struct Rift {
    encoded_world_states: RingBuffer<(usize, Vec<u8>)>,
    latest_game_state: Option<World>,
    recv_from_server: Receiver<UpdateWorldMessage>,
    send_to_server: Sender<Vec<u8>>,
    previous_inputs: Vec<u8>,
    previous_mouse_inputs: Vec<MouseState>,
    current_input_value: u8,
    current_mouse_input_value: u8,
    last_mouse_click_position: Vector2<f32>,
    camera: Camera,
    render_state: RenderState,
    rune_system: RuneSystem
}

impl Rift {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> Rift {
        let (recv_from_server, send_to_server) = start_player_thread(String::from("127.0.0.1:34255"));
        Rift {
            encoded_world_states: RingBuffer::new(),            
            latest_game_state: None,
            recv_from_server,
            send_to_server,
            previous_inputs: vec![],
            previous_mouse_inputs: vec![],
            current_input_value: 0,
            current_mouse_input_value: 0,
            last_mouse_click_position: Vector2::new(0.0, 0.0),
            camera: Camera::new(ctx),
            render_state: RenderState::new(ctx),
            rune_system: RuneSystem::new()
        }
    }

    pub fn handle_message_update(&mut self, mut message: UpdateWorldMessage) {
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
                    let mut encoded: Vec<u8> = bincode::encode_to_vec(&awk_frame_message, config).unwrap();
                    encoded.insert(0, awk_frame_message.message_type.to_u8());
                    let _ = self.send_to_server.send(encoded);
                    self.encoded_world_states.add_new_data((message.current_frame_number, message.data.clone()));

                    if self.latest_game_state.is_none() || self.latest_game_state.as_ref().unwrap().frame_number <= message.current_frame_number {

                        let world : World = World::new_from_byte_array(message.data);
                        
                        self.latest_game_state = Some(world);
                    }
                }
                else {
                    //If we could not find a frame to build the delta off of, just ignore it
                    println!("NOTNAKJSDLAKSJDALSKDJ");
                }
            },
            ToPlayerMessageType::StateWorld => {
                let world : World = World::new_from_byte_array((&message.data[..]).to_vec());
                self.encoded_world_states.add_new_data((message.current_frame_number, message.data));
                let awk_frame_message = AwkFrameMessage::new(message.current_frame_number);
                let mut encoded: Vec<u8> = bincode::encode_to_vec(&awk_frame_message, config).unwrap();
                encoded.insert(0, awk_frame_message.message_type.to_u8());
                let _ = self.send_to_server.send(encoded);
                self.latest_game_state = Some(world);
            }
        }
    }



    pub fn update(&mut self, ctx: &mut Context<AgmaClientApp>, delta: f32) {
        ctx.clear(ClearMode::new().with_color(RGBA8::BLUE).with_depth(0.0, DepthTest::Greater));
        // the message, it will be cut off.
        let config = config::standard();
        let from_server_messages : Vec<UpdateWorldMessage> = self.recv_from_server.try_iter().collect();
        for message in from_server_messages {
            self.handle_message_update(message);
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
        let mut encoded: Vec<u8> = bincode::encode_to_vec(&to_server_input_message, config).unwrap();
        encoded.insert(0, to_server_input_message.message_type.to_u8());
        let _ = self.send_to_server.send(encoded);

        if self.latest_game_state.is_some() {
            let plane_intercept = self.camera.point_on_floor_plane(storm::cgmath::Vector2::new(self.last_mouse_click_position.x, self.last_mouse_click_position.y));
            self.previous_mouse_inputs.push(MouseState::new(self.current_mouse_input_value != 0, plane_intercept));
            let to_server_mouse_input_message = MouseActionMessage::new(self.previous_mouse_inputs.clone());
            //self.latest_game_state.as_mut().unwrap().click_inputs = self.previous_mouse_inputs.iter().map(|x|WorldMouseState::new(x)).collect();
            let mut encoded: Vec<u8> = bincode::encode_to_vec(&to_server_mouse_input_message, config).unwrap();
            encoded.insert(0, to_server_mouse_input_message.message_type.to_u8());
            let _ = self.send_to_server.send(encoded);
        }

        
        match &mut self.latest_game_state.as_mut() {
            Some(world) => {
                let mut transforms_maybe = world.borrow_component_vec::<TransformComponent>().unwrap();
                let mut player_component = world.borrow_component_vec::<ChampionComponent>().unwrap();
                let zip = transforms_maybe.iter_mut().zip(player_component.iter_mut());
                let mut player_transform_query = zip.filter_map(|(tc, cc)|Some((tc.as_mut()?, cc.as_mut()?)));
                for (tc, cc) in player_transform_query {
                    self.camera.look_at(tc.position());
                    self.camera.second_update(tc.position());    
                }
            },
            None => {

            }
        }


        self.render_world();
    }
    
    pub fn render_world(&mut self) {
        
        self.render_state.floor_buffer.set(&self.render_state.floor.as_slice());
        self.render_state.texture_shader.draw(&self.camera.model_view_projection_uniform(&Matrix4::from_scale(100.0)), &self.render_state.floor_texture, &self.render_state.floor_buffer);
        

        if self.latest_game_state.is_some() {
            let game_state = self.latest_game_state.as_ref().unwrap();
            {
                //Player rendering system
                let mut transforms_maybe = game_state.borrow_component_vec::<TransformComponent>().unwrap();
                let mut player_component = game_state.borrow_component_vec::<ChampionComponent>().unwrap();
                let zip = transforms_maybe.iter_mut().zip(player_component.iter_mut());
                let mut player_transform_query = zip.filter_map(|(tc, cc)|Some((tc.as_mut()?, cc.as_mut()?)));
                for (tc, cc) in player_transform_query {
                    self.render_state.particle_buffer.set(&self.render_state.cube.as_slice());
                    self.render_state.model_shader.draw(&self.camera.model_view_projection_uniform(&tc.transform), &self.render_state.particle_buffer);
                }
            }
            {
                let mut minion_components = game_state.borrow_component_vec::<MinionComponent>().unwrap();
                let mut transform_components = game_state.borrow_component_vec::<TransformComponent>().unwrap();
    
                let zip = minion_components.iter_mut().zip(transform_components.iter_mut());
    
                let mut move_champions_iter = zip.filter_map(|(mc, tc)|Some((mc.as_mut()?, tc.as_mut()?)));
                for (mc, tc) in move_champions_iter {
                    self.render_state.particle_buffer.set(&self.render_state.cube.as_slice());
                    self.render_state.model_shader.draw(&self.camera.model_view_projection_uniform(&tc.transform), &self.render_state.particle_buffer);
                }
            }


        }
    
    }

    pub fn on_cursor_pressed(
        &mut self,
        _ctx: &mut Context<AgmaClientApp>,
        button: event::CursorButton,
        _physical_pos: storm::cgmath::Vector2<f32>,
        normalized_pos: storm::cgmath::Vector2<f32>,
    ) {
        match button {
            CursorButton::Right => {
                self.current_mouse_input_value |= 1;            
                self.last_mouse_click_position = Vector2::new(normalized_pos.x, normalized_pos.y);
            },
            CursorButton::Left => {
                self.current_mouse_input_value |= 2;
            },
            _ => {

            }
        }
    }

    pub fn on_cursor_released(
        &mut self,
        _ctx: &mut Context<AgmaClientApp>,
        button: event::CursorButton,
        _physical_pos: storm::cgmath::Vector2<f32>,
        _normalized_pos: storm::cgmath::Vector2<f32>,
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

    pub fn on_key_pressed(&mut self, ctx: &mut Context<AgmaClientApp>, key: event::KeyboardButton, is_repeat: bool) {
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

    pub fn on_cursor_delta(&mut self, _ctx: &mut Context<AgmaClientApp>, delta: storm::cgmath::Vector2<f32>, _focused: bool) {
        //self.camera.look(delta);
    }

    pub fn on_key_released(&mut self, _ctx: &mut Context<AgmaClientApp>, key: event::KeyboardButton) {
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