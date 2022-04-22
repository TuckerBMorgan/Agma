use shared_code::*;
use std::time::Duration;
use std::thread::sleep;
use cgmath::*;
use bincode::*;

use log::info;
use log::LevelFilter;
mod components;
pub use components::*;

fn main() {
    let _ = simple_logging::log_to_file("server.log", LevelFilter::Info);
    let mut connections = vec![PlayerConnection::new(String::from("192.0.0.1"))];

    let mut w = World::new();
    w.register_type::<TransformComponent>(true);
    w.register_type::<ChampionComponent>(true);
    w.register_type::<CharacterStateComponent>(true);
    w.register_type::<MinionComponent>(true);
    w.register_type::<PlayerConnectionComponent>(false);
//    let mut rune_system = RuneSystem::new();

    for i in 0..1 {
        let entity = w.new_entity();
        w.add_component_to_entity(entity, TransformComponent::new(Matrix4::from_translation(Vector3::new(1.0f32, 0.0, 0.0))));
        w.add_component_to_entity(entity, CharacterStateComponent::new());
        w.add_component_to_entity(entity, ChampionComponent::new());
        w.add_component_to_entity(entity, PlayerConnectionComponent::new(i));
    }
    
    for _i in 0..2 {
        let entity = w.new_entity();
        w.add_component_to_entity(entity, TransformComponent::new(Matrix4::from_translation(Vector3::new(0.0f32, 0.0, 0.0))));
        w.add_component_to_entity(entity, CharacterStateComponent::new());
        w.add_component_to_entity(entity, MinionComponent::new());
    }
    
    
    loop {

        {
            //Player Connection Input System
            for connection in connections.iter_mut() {
                connection.check_on_player();
            }

            let mut player_connection_components = w.borrow_component_vec::<PlayerConnectionComponent>().unwrap();
            let mut champion_component = w.borrow_component_vec::<ChampionComponent>().unwrap();
            let zip = player_connection_components.iter_mut().zip(champion_component.iter_mut());
            let player_connection_components = zip.filter_map(|(pcc, cc)|Some((pcc.as_mut()?, cc.as_mut()?)));
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
            //Player Movement System
            let mut champion_component = w.borrow_component_vec::<ChampionComponent>().unwrap();
            let mut character_state_component = w.borrow_component_vec::<CharacterStateComponent>().unwrap();
            let mut transform_component = w.borrow_component_vec::<TransformComponent>().unwrap();

            let zip = champion_component.iter_mut().zip(character_state_component.iter_mut()).zip(transform_component.iter_mut());

            let move_champions_iter = zip.filter_map(|((pcc, cc), tc)|Some((pcc.as_mut()?, cc.as_mut()?, tc.as_mut()?)));

            for (cc, mc, tc) in move_champions_iter {
                let current_player_input = cc.get_current_input();
                match current_player_input {
                    Some(mouse_input) => {
                        if mouse_input.button_down {
                            mc.character_state = CharacterState::Moving(Vector3::new(mouse_input.x, mouse_input.y, mouse_input.z));
                        }
                    },
                    None => {

                    }
                }
                match mc.character_state {
                    CharacterState::Moving(location) => {
                        let direction = (location - tc.position()).normalize();
                        tc.move_character(direction * 1.0 * 0.1);
                    },
                    _ => {

                    }
                }
            }
        }


        {
            // Minion movement system
            let mut minion_components = w.borrow_component_vec::<MinionComponent>().unwrap();
            let mut character_state_components = w.borrow_component_vec::<CharacterStateComponent>().unwrap();
            let mut transform_components = w.borrow_component_vec::<TransformComponent>().unwrap();

            let zip = minion_components.iter_mut().zip(character_state_components.iter_mut()).zip(transform_components.iter_mut());

            let move_champions_iter = zip.filter_map(|((pcc, cc), tc)|Some((pcc.as_mut()?, cc.as_mut()?, tc.as_mut()?)));

            for (minion_component, _mc, tc) in move_champions_iter {
                let distance_to_current_position = (tc.position() - minion_component.destinations[minion_component.current_index]).magnitude();
                if distance_to_current_position < 0.01 {
                    minion_component.current_index = (minion_component.current_index + 1) % 2;
                }
                let direction = (tc.position() - minion_component.destinations[minion_component.current_index]).normalize();
                tc.move_character(direction * 1.0  * 0.01);
            }
        }

        /*
        {
            // Auto attack system
            let mut autoattacks = w.borrow_component_vec::<AutoAttackComponent>().unwrap();
            //I want a function for "find every close entity"
            //maybe? hmmm, have the character state component add an auto attack component
            //which in turn will handle all of the stuff
            //which I guess is fine
            //the character does not quite care
            //would care that it is over for sure though
            //which I guess can be done by having the 
            //o I like that
            //becuase then if the character becomes stunned
            //or preforms an interupt action
            //the system can remove the component
            let autoattacks = autoattacks.iter_mut().filter_map(|aac|aac.as_mut()?);
            for aa in autoattacks {
                match aa.state {
                    AutoAttackState::Windup || AutoAttackState::Firing => {
                        aa.current_progress += aa.attack_speed;
                        if aa.current_progress / aa.length_of_auto_attack >= 0.33 {
                            aa.state = AutoAttackState::Firing;
                        }
                        else if aa.current_progress >= aa.length_of_auto_attack {
                            aa.current_progress = aa.length_of_auto_attack;
                            // "Do damage"
                            // "Remove this"??
                        }
                    },
                    _ => {

                    }
                }
            }
            
            
        }
        */
        

        
        for connection in connections.iter_mut() {
            connection.update_player_with_new_game_state(w.to_byte_array(), w.frame_number);
        }
        sleep(Duration::from_millis(14));
    }
}
