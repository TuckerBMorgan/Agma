use shared_code::*;
use std::time::Duration;
use std::thread::sleep;
use bincode::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use rand::Rng;
use log::LevelFilter;

mod components;
pub use components::*;

mod server_managers;
pub use server_managers::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GridSlot {
    Character(usize),
    Empty
}


const GRID_SIZE : usize = 1000;
fn main() {
    
    let mut server_socket = ServerSocket::new();
    let (from_handshake_thread, to_handshake_thread) = start_handshake_thread();

    let _ = simple_logging::log_to_file("server.log", LevelFilter::Info);
    let mut connections : HashMap<SocketAddr, PlayerConnection> = HashMap::new();
    let mut client_id_to_socket_address : HashMap<usize, SocketAddr> = HashMap::new();
    let mut placed_characters = vec![GridSlot::Empty;GRID_SIZE * GRID_SIZE];
    let mut entity_id_to_position_map;// = HashMap::new();


    let mut w = make_basic_world();
    // Put components you don't want replciated here
    w.register_type::<PlayerConnectionComponent>(false);
    let mut rune_system = RuneSystem::new();
    let ability_function_library = load_basic_functions();
    let mut ability_instance_library = AbilityInstanceLibrary::new();
    let mut ability_target_functions = AbilityTargetFunctionLibrary::new();
    let target_is_position_function = ability_target_functions.add_function(target_is_position);
    let mut champions : HashMap<usize, Champion> = HashMap::new();
    
    for i in 0..2 {
        let entity = w.new_entity();
        w.add_component_to_entity(entity, PositionComponent::new(20, (i + 1) * 20));
        w.add_component_to_entity(entity, CharacterStateComponent::new());
        w.add_component_to_entity(entity, MinionComponent::new());
        w.add_component_to_entity(entity, TeamComponent::new(1));
        w.add_component_to_entity(entity, HealthComponent::new(100));
        w.add_component_to_entity(entity, RadiusComponent::new(1));
        w.add_component_to_entity(entity, ZomebieControllerComponent::new());
        let movement_ability = AbilityInstance::new(0, 12, BASIC_MOVEMENT_ABILITY_FUNCTION_ID, target_is_position_function, 10000);
        let autoattack_ability = AbilityInstance::new(30, 10, BASIC_MOVEMENT_ABILITY_FUNCTION_ID, target_is_position_function, 1);
        
        let movement_ability_id = ability_instance_library.add_new_ability_instance(movement_ability);
        let autoattack_ability_id = ability_instance_library.add_new_ability_instance(autoattack_ability);
        w.add_component_to_entity(entity, AbilityComponent::new([movement_ability_id, autoattack_ability_id, NULL_ABILITY_VALUE, NULL_ABILITY_VALUE]));
    }

    loop {
        {
            //Handle new players connecting to the game
            let join_requests = from_handshake_thread.try_iter();
            for join_request in join_requests {
                //TODO: turn this into a function I can call
                let client_id = connections.len();
                let unique_port = 4560 + client_id as u16;
                let socket = SocketAddr::new(join_request.socket_address.ip(), unique_port);
                let foo = PlayerConnection::new(socket);
                let entity = w.new_entity();

                let champion = Champion::new(client_id as u8);
                champions.insert(client_id, champion);
                w.add_component_to_entity(entity, PositionComponent::new(0, 0));
                w.add_component_to_entity(entity, CharacterStateComponent::new());
                w.add_component_to_entity(entity, ChampionComponent::new(client_id as u8));
                w.add_component_to_entity(entity, PlayerConnectionComponent::new(client_id));
                w.add_component_to_entity(entity, TeamComponent::new(0));
                w.add_component_to_entity(entity, HealthComponent::new(100));
                w.add_component_to_entity(entity, RadiusComponent::new(1));

                let movement_ability = AbilityInstance::new(0, 12, BASIC_MOVEMENT_ABILITY_FUNCTION_ID, target_is_position_function, 10000);
                let autoattack_ability = AbilityInstance::new(30, 10, AUTOATTACK_ABILITY_FUNCTION_ID, target_is_position_function, 1);
                
                let movement_ability_id = ability_instance_library.add_new_ability_instance(movement_ability);
                let autoattack_ability_id = ability_instance_library.add_new_ability_instance(autoattack_ability);
                w.add_component_to_entity(entity, AbilityComponent::new([movement_ability_id, autoattack_ability_id, NULL_ABILITY_VALUE, NULL_ABILITY_VALUE]));

                let _ = to_handshake_thread.send(PlayerConnectionInfo::new(join_request.stream_id, client_id as u8, unique_port));
                connections.insert(socket, foo);
                client_id_to_socket_address.insert(client_id, socket);
            }

            // drain out socket and update the player connection info with the latest information
            server_socket.check_on_players(&mut connections);


            //clear the grid so we can recalculate everything
            for i in 0..(GRID_SIZE * GRID_SIZE) {
                placed_characters[i] = GridSlot::Empty;
            }

            entity_id_to_position_map = HashMap::new();
            let entity_position_query;            
            query_3!(EntityComponent, PositionComponent, HealthComponent, w, entity_position_query);
            for (ec, pc, hc) in entity_position_query {
                if hc.current_amount > 0 {
                    let effective_x = pc.x as usize + GRID_SIZE / 2;
                    let effective_y = pc.y as usize + GRID_SIZE / 2;
                    placed_characters[effective_x + effective_y * GRID_SIZE] = GridSlot::Character(ec.id);
                    entity_id_to_position_map.insert(ec.id, (pc.x, pc.y));
                }
            }
            
            let player_connection_components;
            query_2!(PlayerConnectionComponent, ChampionComponent, w, player_connection_components);
            for (pcc, cc) in player_connection_components {
                let socket_address = client_id_to_socket_address[&pcc.player_index];
                let player_connection = &connections[&socket_address];
                if player_connection.desired_inputs.len() >= 16 {
                    let mut champion = champions.get_mut(&(cc.champion_index as usize)).unwrap();
                    for i in 0..16 {
                        champion.desired_inputs[i] = player_connection.desired_inputs[i];
                    }
                    champion.current_input_to_use = 0;
                }
            }
        }

        {
            let target_request_query;
            query_2!(EntityComponent, ChampionComponent, w, target_request_query);
            for (ec, cc) in target_request_query {
                let mut champion = champions.get_mut(&(cc.champion_index as usize)).unwrap();
                let input = champion.get_current_input();
                if input.is_none() {
                    continue;
                }

                let input = input.unwrap();
                if input.button_down {
                    //TODO: I hate all of this 
                    let input_x = input.x.round() as i64;
                    let input_y = input.z.round() as i64;
                    let effective_x = input.x.round() as usize + GRID_SIZE / 2;
                    let effective_y = input.z.round() as usize + GRID_SIZE / 2;
                    let clicked_index = effective_x + effective_y * GRID_SIZE;
                    match placed_characters[clicked_index] {
                        GridSlot::Character(id) => {
                            if w.are_friends(id, ec.id) == false {
                                champion.desired_action = DesiredAction::Attack(id);
                            }
                        },_ => {
                            //TODO: will need to expand this to mean maybe "target position", where if you are not using an ability
                            champion.desired_action = DesiredAction::MoveToPosition(input_x, input_y);
                        }
                    }
                }
            }
        }


        {

            //TODO: think more about this block below here, something seems off
            let process_champion_desired_action_query;
            query_4!(EntityComponent, ChampionComponent, AbilityComponent, PositionComponent, w, process_champion_desired_action_query);
            for (ec, cc, ac, pc) in process_champion_desired_action_query {
                let mut champion = champions.get_mut(&(cc.champion_index as usize)).unwrap();
                match &champion.desired_action {
                    DesiredAction::Attack(target_id) => {
                        let mut out_of_range = false;
                        {
                            //Attempt to autoattack the target
                            let active_ability_instance = ability_instance_library.checkout_ability_instance(ac.ability_ids[ac.active_ability]);
                            match active_ability_instance {
                                Some(ability_instance) => {
                                    if ability_instance.ability_state == AbilityState::Ready {
                                        let ability_target_instance = AbilityTargetContext::new(&w, ec.id, ability_instance, AbilityTarget::Character(*target_id));
                                        if ability_target_functions.call_target_validator_function(ability_instance.target_validator, ability_target_instance) {
                                            ability_instance.start_with_target(AbilityTarget::Character(*target_id));
                                        }
                                        else {
                                            out_of_range = true;
                                        }                                            
                                    }
                                },
                                None => {
                                }
                            }
                        }
                        //If we are out of range move one closer to the character
                        if out_of_range {
                            let active_ability_instance = ability_instance_library.checkout_ability_instance(ac.ability_ids[MOVEMENT_ABILITY_INDEX]);
                            match active_ability_instance {
                                Some(ability_instance) => {
                                    if ability_instance.ability_state == AbilityState::Ready {
                                        let (target_x, target_y) = entity_id_to_position_map[target_id];
                                        let x_direction = (target_x - pc.x).signum();
                                        let y_direction = (target_y - pc.y).signum();
                                        ability_instance.start_with_target(AbilityTarget::Position(pc.x + x_direction, pc.y + y_direction));
                                    }
                                },
                                None => {
                                }
                            }

                        }
                    },
                    DesiredAction::MoveToPosition(x, y) => {
                        if *x != pc.x || *y != pc.y {
                            match ability_instance_library.checkout_ability_instance(ac.ability_ids[MOVEMENT_ABILITY_INDEX]) {
                                Some(ability_instance) => {                                
                                    if ability_instance.ability_state == AbilityState::Ready {
                                        let x_direction = (x - pc.x).signum();
                                        let y_direction = (y - pc.y).signum();
                                        ability_instance.start_with_target(AbilityTarget::Position(pc.x + x_direction, pc.y + y_direction));
                                    }
                                },
                                None => {
    
                                }
                            }
                        } 
                        else {
                            champion.desired_action = DesiredAction::Nothing;
                        }

                    },
                    DesiredAction::Nothing => {
                        //Do nothing
                    }
                }
            }
        }

        //Tick forward any active ability
        let mut ability_activation_requests = vec![];
        {
            let ability_update_query;
            query_2!(EntityComponent, AbilityComponent, w, ability_update_query);
            for (ec, ac) in ability_update_query {
                for ability_instance_id in ac.ability_ids {
                    if ability_instance_id == 0 {
                        //0 is used as a flag value
                        continue;
                    }
                    match ability_instance_library.checkout_ability_instance(ability_instance_id) {
                        Some(ability_instance) => {
                            match ability_instance.ability_state {
                                AbilityState::Channeling | AbilityState::Recovering => {
                                    //TODO: do this else where, make me unhappy
                                    if ability_instance.advance() {
                                        //We now have an ability to activate
                                        ability_activation_requests.push((ec.id, ability_instance_id));
                                    }
                                },
                                _ => {

                                }
                            }
                        },
                        None => {
                            panic!("Ability {} not found for character", ability_instance_id);
                        }
                    }
                }
            }
        }

        //Actually process those abilities that have reached the end of their channel
        for (entity_id, ability_instance_id) in ability_activation_requests {
            let mut ability_instance = ability_instance_library.checkout_ability_instance(ability_instance_id).unwrap();
            let function_id = ability_instance.action;
            let context = AbilityContext::new(&mut w, entity_id, &mut rune_system, &mut ability_instance);
            ability_function_library.call_action_function(function_id, context);
        } 
        /*
        {
            let zombie_controller_query;
            query_3!(ZomebieControllerComponent, MovementStateComponent, PositionComponent, w, zombie_controller_query);
            for (zcc, msc, pc) in zombie_controller_query {
                match zcc.state {
                    ZomebieState::Idle{walk_timer} => {
                        if walk_timer + 1 == ZOMBIE_WALK_TIMER_MAX {
                            zcc.state = ZomebieState::Idle{walk_timer: 0};
                            let mut rng = rand::thread_rng();

                            let initial : i64 = rng.gen_range(0..3);
                            let x_offset = initial - 1;
                            let initial : i64 = rng.gen_range(0..3);
                            let mut y_offset = initial - 1;
                            if y_offset == 0 && x_offset == 0 {
                                y_offset += 1;
                            }
                            msc.start_moving(pc.x + x_offset, pc.y + y_offset);
                        }
                        else {
                            zcc.state = ZomebieState::Idle{walk_timer: (walk_timer + 1)};
                        }
                    },
                    _ => {

                    }
                }
            }
        }
        */
        {
            let dead_things_query;
            query_2!(EntityComponent, HealthComponent, w, dead_things_query);
            for (ec, hc) in dead_things_query {
                if hc.current_amount ==  0 {
                    ec.in_use = false;
                }
            }
        }
        



        rune_system.resolve_world_state(&mut w);
        w.cleanup_world();
        for (_key, connection) in connections.iter_mut() {
            connection.update_player_with_new_game_state(w.to_byte_array(), w.frame_number, &mut server_socket.socket);
        }
        sleep(Duration::from_millis(14));
    }
}
