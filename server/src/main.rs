use shared_code::*;
use std::time::Duration;
use std::thread::sleep;
use bincode::*;
use std::cmp;
use std::collections::HashMap;


use log::info;
use log::LevelFilter;

mod components;
pub use components::*;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GridSlot {
    Character(usize),
    Empty
}


const GRID_SIZE : usize = 1000;
fn main() {
    let _ = simple_logging::log_to_file("server.log", LevelFilter::Info);
    let mut connections = vec![PlayerConnection::new(String::from("192.0.0.1"))];

    let mut placed_characters = vec![GridSlot::Empty;GRID_SIZE * GRID_SIZE];
    let mut entity_id_to_position_map = HashMap::new();

    let mut w = World::new();
    // put components you want replicated here
    w.register_type::<EntityComponent>(true);
    w.register_type::<PositionComponent>(true);
    w.register_type::<ChampionComponent>(true);
    w.register_type::<CharacterStateComponent>(true);
    w.register_type::<MinionComponent>(true);
    w.register_type::<TeamComponent>(true);
    w.register_type::<HealthComponent>(true);
    w.register_type::<RadiusComponent>(true);
    w.register_type::<MovementStateComponent>(true);
    w.register_type::<AttackStateComponent>(true);

    // Put components you don't want replciated here
    w.register_type::<PlayerConnectionComponent>(false);
    let mut rune_system = RuneSystem::new();

    for i in 0..1 {
        let entity = w.new_entity();
        w.add_component_to_entity(entity, PositionComponent::new(0, 0));
        w.add_component_to_entity(entity, CharacterStateComponent::new());
        w.add_component_to_entity(entity, ChampionComponent::new());
        w.add_component_to_entity(entity, PlayerConnectionComponent::new(i));
        w.add_component_to_entity(entity, TeamComponent::new(0));
        w.add_component_to_entity(entity, HealthComponent::new(100));
        w.add_component_to_entity(entity, RadiusComponent::new(1));
        w.add_component_to_entity(entity, MovementStateComponent::new(12));
        w.add_component_to_entity(entity, AttackStateComponent::new(30, 1, 30));
    }
    
    for i in 0..2 {
        let entity = w.new_entity();
        w.add_component_to_entity(entity, PositionComponent::new(20, (i + 1) * 20));
        w.add_component_to_entity(entity, CharacterStateComponent::new());
        w.add_component_to_entity(entity, MinionComponent::new());
        w.add_component_to_entity(entity, TeamComponent::new(1));
        w.add_component_to_entity(entity, HealthComponent::new(100));
        w.add_component_to_entity(entity, RadiusComponent::new(1));
        w.add_component_to_entity(entity, MovementStateComponent::new(1));
        w.add_component_to_entity(entity, AttackStateComponent::new(30, 1, 240));
    }
    

    loop {

        {
            //Player Connection Input System
            for connection in connections.iter_mut() {
                connection.check_on_player();
            }

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
                let player_connection = &connections[pcc.player_index];
                if player_connection.desired_inputs.len() >= 16 {
                    for i in 0..16 {
                        cc.desired_inputs[i] = player_connection.desired_inputs[i];
                    }
                    cc.current_input_to_use = 0;
                }
            }
        }


        {
            let player_movement_query;
            query_4!(ChampionComponent, MovementStateComponent, PositionComponent, AttackStateComponent, w, player_movement_query);
            for (cc, msc, pc, asc) in player_movement_query {

                let input = cc.get_current_input();
                if input.is_none() {
                    continue;
                }

                let input = input.unwrap();
                let input_x = input.x.round() as i64;
                let input_y = input.z.round() as i64;
                if input.button_down {                    
                    let effective_x = input.x.round() as usize + GRID_SIZE / 2;
                    let effective_y = input.z.round() as usize + GRID_SIZE / 2;
                    let clicked_index = effective_x + effective_y * GRID_SIZE;
                    //If you click on "yourself" we don't do anything
                    if input_x != pc.x || input_y != pc.y {
                        if asc.is_attacking == false {
                            match placed_characters[clicked_index] {
                                GridSlot::Character(id) => {
                                    let distance : usize = cmp::max((input_x - pc.x).abs(), (input_y - pc.y).abs()) as usize;
                                    if distance <= asc.range {
                                        asc.start_attacking(id, input_x, input_y);
                                        msc.is_moving = false;    
                                    }
                                    else {
                                        msc.start_attack_moving(id, asc.range);
                                    }
                                },_ => {
                                    if input_x != pc.x || input_y != pc.y {
                                        msc.start_moving(input_x, input_y);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        {
            let character_movement_query;
            query_3!(EntityComponent, MovementStateComponent, PositionComponent, w, character_movement_query);
            for (ec, msc, pc) in character_movement_query {

                if msc.current_move_speed < msc.move_speed {
                    msc.current_move_speed += 1;
                }

                if msc.current_move_speed > msc.move_speed {
                    msc.move_speed = msc.move_speed;
                }

                match msc.movement_type {
                    MovementType::AttackMove(id, maximum_range) => {
                        let (target_position_x, target_position_y) = entity_id_to_position_map[&id];
                        if game_distance_between_two_points(pc.x, pc.y, target_position_x, target_position_y) <= maximum_range {
                            msc.is_moving = false;
                            let mut attack_state_component = w.borrow_component_vec::<AttackStateComponent>().unwrap()[id].unwrap();
                            attack_state_component.start_attacking(id, target_position_x, target_position_y);
                        }
                        else {
                            msc.destination_x = target_position_x;
                            msc.destination_y = target_position_y;    
                        }
                    },
                    _ => {
                        
                    }
                }
                


                if msc.is_moving {
                    if msc.current_move_speed == msc.move_speed  {
                        msc.current_move_speed = 0;
                        if pc.x != msc.destination_x || pc.y != msc.destination_y {
                            let mut x_update = msc.destination_x - pc.x;
                            if x_update != 0 {
                                x_update = x_update.signum();
                            }

                            let mut y_update = msc.destination_y - pc.y;
                            if y_update != 0 {
                                y_update = y_update.signum();
                            }

                            pc.update_position(x_update, y_update);
                            if pc.x == msc.destination_x && pc.y == msc.destination_y {
                                msc.is_moving = false;
                            }
                        }
                    }
                }
            }
        }

        
        {
            let character_attack_query;
            query_2!(EntityComponent, AttackStateComponent, w, character_attack_query);
            for (ec, asc) in character_attack_query {
                if asc.is_attacking {
                    if asc.current_channel < asc.channel_timer {
                        asc.current_channel += 1;
                    }

                    if asc.current_channel >= asc.channel_timer {
                        asc.is_attacking = false;
                        asc.current_channel = 0;
                        let damage_rune = DamageRune::new(ec.id, asc.current_target, 10);
                        rune_system.add_rune(damage_rune.into());
                    }
                }
            }
        } 
        
        

        rune_system.resolve_world_state(&mut w);

        for connection in connections.iter_mut() {
            connection.update_player_with_new_game_state(w.to_byte_array(), w.frame_number);
        }
        sleep(Duration::from_millis(14));
    }
}
