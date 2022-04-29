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
    // put components you want replicated here
    w.register_type::<EntityComponent>(true);
    w.register_type::<TransformComponent>(true);
    w.register_type::<ChampionComponent>(true);
    w.register_type::<CharacterStateComponent>(true);
    w.register_type::<MinionComponent>(true);
    w.register_type::<AutoAttackComponent>(true);
    w.register_type::<TeamComponent>(true);
    w.register_type::<HealthComponent>(true);

    // Put components you don't want replciated here
    w.register_type::<PlayerConnectionComponent>(false);
    let mut rune_system = RuneSystem::new();

    for i in 0..1 {
        let entity = w.new_entity();
        w.add_component_to_entity(entity, TransformComponent::new(Matrix4::from_translation(Vector3::new(1.0f32, 0.0, 0.0))));
        w.add_component_to_entity(entity, CharacterStateComponent::new());
        w.add_component_to_entity(entity, ChampionComponent::new());
        w.add_component_to_entity(entity, PlayerConnectionComponent::new(i));
        w.add_component_to_entity(entity, TeamComponent::new(0));
    }
    
    for _i in 0..2 {
        let entity = w.new_entity();
        w.add_component_to_entity(entity, TransformComponent::new(Matrix4::from_translation(Vector3::new(0.0f32, 0.0, 0.0))));
        w.add_component_to_entity(entity, CharacterStateComponent::new());
        w.add_component_to_entity(entity, MinionComponent::new());
        w.add_component_to_entity(entity, TeamComponent::new(1));
    }

    loop {

        {
            //Player Connection Input System
            for connection in connections.iter_mut() {
                connection.check_on_player();
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
            query_4!(EntityComponent, ChampionComponent, CharacterStateComponent, TransformComponent, w, player_movement_query);
            // Check to see if any of the players are moving this frame
            for (ec, cc, mc, tc) in player_movement_query {
                let current_player_input = cc.get_current_input();
                match current_player_input {
                    Some(mouse_input) => {
                        //Did the player "click" in this frame
                        if mouse_input.button_down && mouse_input.was_down == false {
                            // we now need to check, did they click on open ground, or a character
                            let what_did_they_click_on = w.entity_at_point(Vector2::new(mouse_input.x, mouse_input.y));
                            match what_did_they_click_on {
                                Some(entity_id) => {
                                    if w.are_friends(ec.id, entity_id) == false {
                                        match mc.character_state {
                                            //If you are currently autoattacking, only change the 
                                            CharacterState::AutoAttacking(auto_attack_state) => {
                                                //Don't interupt an autoattack that is targeting the same person
                                                if auto_attack_state.target != entity_id {
                                                    //Maybe need to replace?
                                                    //also this would let a player interupt the actual firing state, which would be undesirable
                                                    mc.character_state = CharacterState::AutoAttacking(AutoAttackState::new(entity_id, 0));
                                                }
                                            },
                                            _ => {
                                                mc.character_state = CharacterState::AutoAttacking(AutoAttackState::new(entity_id, 0));
                                            }
                                        }
                                    }
                                    else {
                                        //If the entity we clicked on was friendly, we simply will walk towards it then
                                        mc.character_state = CharacterState::Moving(Vector3::new(mouse_input.x, mouse_input.y, mouse_input.z));
                                    }
                                },
                                None => {
                                    mc.character_state = CharacterState::Moving(Vector3::new(mouse_input.x, mouse_input.y, mouse_input.z));
                                }
                            }
                        }
                    },
                    None => {

                    }
                }
                // For any character that is moving issue a move command to be handled latter by the rune system
                match mc.character_state {
                    CharacterState::Moving(location) => {
                        let direction = (location - tc.position()).normalize();
                        tc.move_character(direction * 1.0 * 0.1);

                        let move_rune = MoveRune::new(ec.id, direction);
                        rune_system.add_rune(move_rune.into());
                    },
                    CharacterState::AutoAttacking(autoatack_attack) => {
                        autoatack_attack.progress += 1;
                        if autoatack_attack >= 100 {
                            let damage_rune = DamageRune::new(autoatack_attack.target, 10);
                            rune_system.add_rune(damage_rune.into());
                            mc.character = CharacterState::Idle;
                        }
                    }
                    _ => {

                    }
                }
            }
        }


        //TODO: move this into a the above part
        //And move this "MinionComponet" to be more about following a lane
        {
            let minion_move_iter;
            query_4!(EntityComponent, MinionComponent, CharacterStateComponent, TransformComponent, w, minion_move_iter);

            for (entity_component, minion_component, _mc, tc) in minion_move_iter {
                let distance_to_current_position = (tc.position() - minion_component.destinations[minion_component.current_index]).magnitude();
                if distance_to_current_position < 0.01 {
                    minion_component.current_index = (minion_component.current_index + 1) % 2;
                }
                let direction = (tc.position() - minion_component.destinations[minion_component.current_index]).normalize();
                let move_rune = MoveRune::new(entity_component.id, direction);
                rune_system.add_rune(move_rune.into());
            }
        }
        
        

        rune_system.resolve_world_state(&mut w);
        for connection in connections.iter_mut() {
            connection.update_player_with_new_game_state(w.to_byte_array(), w.frame_number);
        }
        sleep(Duration::from_millis(14));
    }
}
