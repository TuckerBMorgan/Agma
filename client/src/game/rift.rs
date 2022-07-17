use crate::*;
use std::sync::mpsc::{Sender, Receiver};
use crate::networking::*;
use crate::rendering::*;
use bincode::{config};
use std::ops::Mul;
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};


pub struct Rift {
    encoded_world_states: RingBuffer<(usize, Vec<u8>)>,
    game_state: World,
    recv_from_server: Receiver<UpdateWorldMessage>,
    send_to_server: Sender<Vec<u8>>,
    previous_inputs: Vec<u8>,
    previous_mouse_inputs: Vec<MouseState>,
    current_input_value: u8,
    current_mouse_input_value: u8,
    was_mouse_button_down_last_frame: bool,
    last_mouse_click_position: Vector2<f32>,
    camera: NewCamera,
    render_state: RenderState,
    ui_render_state: UIRenderState,
    animation_timer: f32,
    last_frames_entities: HashSet<usize>,
    client_id: u8
}

impl Rift {
    pub fn new(ctx: &mut Context<AgmaClientApp>, client_id: u8, port: u16) -> Rift {
        //TODO: this up address needs to be where the server is
        let (recv_from_server, send_to_server) = start_player_thread(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 20)), port));

        let w = make_basic_world();
        
        Rift {
            encoded_world_states: RingBuffer::new(),            
            game_state: w,
            recv_from_server,
            send_to_server,
            previous_inputs: vec![],
            previous_mouse_inputs: vec![],
            current_input_value: 0,
            current_mouse_input_value: 0,
            was_mouse_button_down_last_frame: false,
            last_mouse_click_position: Vector2::new(0.0, 0.0),
            camera: NewCamera::new(ctx),
            render_state: RenderState::new(ctx),
            ui_render_state: UIRenderState::new(ctx),
            animation_timer: 0.0,
            last_frames_entities: HashSet::new(),
            client_id
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

                    if self.game_state.frame_number <= message.current_frame_number {
                        self.game_state.rebuild_world(message.data);
                    }
                }
                else {
                    //If we could not find a frame to build the delta off of, just ignore it
                    println!("NOTNAKJSDLAKSJDALSKDJ");
                }
            },
            ToPlayerMessageType::StateWorld => {
                self.game_state.rebuild_world((&message.data[..]).to_vec());
                self.encoded_world_states.add_new_data((message.current_frame_number, message.data));
                let awk_frame_message = AwkFrameMessage::new(message.current_frame_number);
                let mut encoded: Vec<u8> = bincode::encode_to_vec(&awk_frame_message, config).unwrap();
                encoded.insert(0, awk_frame_message.message_type.to_u8());
                let _ = self.send_to_server.send(encoded);

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


        let plane_intercept = self.camera.point_on_floor_plane(storm::cgmath::Vector2::new(self.last_mouse_click_position.x, self.last_mouse_click_position.y));

        self.previous_mouse_inputs.push(MouseState::new(self.current_mouse_input_value != 0, self.was_mouse_button_down_last_frame, plane_intercept));
        let to_server_mouse_input_message = MouseActionMessage::new(self.previous_mouse_inputs.clone());

        let mut encoded: Vec<u8> = bincode::encode_to_vec(&to_server_mouse_input_message, config).unwrap();
        encoded.insert(0, to_server_mouse_input_message.message_type.to_u8());
        let _ = self.send_to_server.send(encoded);

        



        self.setup_world();
        self.camera.update(delta);
        self.render_world(delta);

        if self.current_mouse_input_value > 0 {
            self.was_mouse_button_down_last_frame = true;
        }
        else {
            self.was_mouse_button_down_last_frame = false;
        }

        {
            self.last_frames_entities = HashSet::new();
            let entity_with_transform;
            query_2!(EntityComponent, TransformComponent, self.game_state, entity_with_transform);
            for (ec, _tc) in entity_with_transform {
                self.last_frames_entities.insert(ec.id);
            }
        }

    }

    pub fn setup_world(&mut self) {
        let mut transform_to_add = vec![];
        {
            let entity = self.game_state.borrow_component_vec::<EntityComponent>().unwrap();
            for ent in entity.iter() {
                match ent {
                    Some(ent) => {
                        if self.last_frames_entities.contains(&ent.id) == false {
                            transform_to_add.push(ent.id);
                        }        
                    },
                    _ => {
                        
                    }
                }
            }
        }

        {
            for id in transform_to_add {
                self.game_state.add_component_to_entity(id, TransformComponent::new(Vector3::new(0.0, 0.0, 0.0), 0.0, Vector3::new(1.0, 1.0, 1.0)));
            }
        }
        

        {
            
            let entity_transform_system;
            query_2!( PositionComponent, TransformComponent, self.game_state, entity_transform_system);
            for (pc,  tc) in entity_transform_system {
                let player_x = pc.x as f32;
                let player_y = pc.y as f32;
                tc.set_desired_translation(Vector3::new(player_x, 0.0, player_y));
                //TODO: handle direction
                /*
                if msc.is_moving {

                let direction_along_x = pc.x as f32 - tc.position().x;
                let direction_along_y = pc.y as f32 - tc.position().z;

                let moving_direction = Vector2::new(direction_along_x as f32, direction_along_y as f32).normalize();

                let player_x = pc.x as f32;
                let player_y = pc.y as f32;


                if (moving_direction.x != 0.0 || moving_direction.y != 0.0) && (tc.translation.desired_translation.x != pc.x as f32 || tc.translation.desired_translation.z != pc.y as f32) {
                    tc.set_desired_translation(Vector3::new(player_x, 0.0, player_y));
                    let mut desired = storm::cgmath::Deg::atan2(moving_direction.x, moving_direction.y).0;

                    if desired > 360.0 {
                        desired = desired % 360.0;
                    }
                    if desired < 0.0 {
                        desired = 360.0 + desired;
                    }


                    tc.set_desired_rotation(desired);
                }

                */
                tc.update_transform();
            }
            
        }

        let camera_position_system;
        {
            query_2!(ChampionComponent, TransformComponent, self.game_state, camera_position_system);
            for (cc, tc) in camera_position_system {
                if cc.champion_index == self.client_id {
                    self.camera.update_player_position(tc.position());
                }
            }
        }



    }
    
    pub fn render_world(&mut self, delta: f32) {
        self.render_state.floor_buffer.set_data(&self.render_state.floor.as_slice());
        self.render_state.texture_shader.draw(&self.camera.model_view_projection_uniform(&Matrix4::from_scale(100.0)), &self.render_state.floor_texture, &self.render_state.floor_buffer);

        {
            //Player rendering system
            let entity_animation_component;
            query_2!(EntityComponent, TransformComponent, self.game_state, entity_animation_component);

            for (ec, tc) in entity_animation_component {
                if ec.in_use == false {
                    continue;
                }
                let mut use_animation = String::from("Idle");
                let mut length_along_animation = 0.0f32;

                //TODO: fiddle with this number to something better
                if  tc.get_translation_offset_magnitude_squared() > 0.01f32 {
                    use_animation = String::from("Running");
                    self.animation_timer += delta;
                }

                if self.animation_timer > self.render_state.skinned_animation_library.loaded_animations[&use_animation].length_of_animation {
                    self.animation_timer = 0.0;
                }
                
                length_along_animation = self.render_state.skinned_animation_library.loaded_animations[&use_animation].length_of_animation * length_along_animation;
                let animated_transform = self.camera.transform.matrix().mul(tc.matrix());
                let test =  self.render_state.skinned_animation_library.loaded_animations.get_mut(&use_animation).as_mut().unwrap().calculate_joint_matrix(length_along_animation);
                self.render_state.skinned_shader_pass.set_uniform(animated_transform, test);
                self.render_state.skinned_shader_pass.buffer.set_data(self.render_state.skinned_animation_library.loaded_animations[&use_animation].model.as_slice()); 
                self.render_state.skinned_shader_pass.draw(&self.render_state.model_shader);
            }
        }
        
        {
            let human_health_component_system;
            query_2!(ChampionComponent, HealthComponent, self.game_state, human_health_component_system);
            for (cc, hc) in human_health_component_system {
                if cc.champion_index == self.client_id { 
                    self.ui_render_state.configure_player_health_bar(hc.current_amount as f32 / 100.0);
                }
            }
        }
    
        self.ui_render_state.render_ui();
    
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

    pub fn on_key_pressed(&mut self, ctx: &mut Context<AgmaClientApp>, key: event::KeyboardButton, _is_repeat: bool) {

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
                self.camera.multiplier += 8.0;
            }
            _ => {}
        }
    }

    pub fn on_cursor_delta(&mut self, _ctx: &mut Context<AgmaClientApp>, _delta: storm::cgmath::Vector2<f32>, _focused: bool) {
      //  self.camera.look(delta);
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
                self.camera.multiplier -= 8.0;
            }
            _ => {}
        }
    }   
}