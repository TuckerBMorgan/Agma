use shared_code::*;
use std::time::Duration;
use std::thread::sleep;
use cgmath::*;
use bincode::*;

use log::info;
use log::LevelFilter;

mod components;
pub use components::*;

pub struct Circle {
    entity: usize,
    center: Vector3<f32>,
    radius: f32,
    radius_squared: f32
}

impl Circle {
    pub fn new(entity: usize, center: Vector3<f32>, radius: f32) -> Circle {
        Circle {
            entity,
            center,
            radius,
            radius_squared: radius * radius
        }
    }

    pub fn point_inside(&self, point: &Vector3<f32>) -> bool {
        let distance = (point - self.center).magnitude2();
        return distance < self.radius_squared;
    }

}


fn overllaped_entity(world_position: Vector2<f32>, caluclated_positions: &Vec<Circle>) -> Option<usize> {
    let point = Vector3::new(world_position.x, 0.0, world_position.y);
    let mut lowest = std::f32::MAX;
    let mut id = 0;

    for circle in caluclated_positions.iter() {
        if circle.point_inside(&point) {
            let distance_between = (circle.center - point).magnitude2();
            if distance_between < lowest {
                lowest  = distance_between;
                id = circle.entity;
            }
        }
    }

    if lowest == std::f32::MAX {
        return None;
    }
    
    return Some(id);

}

fn main() {
    let _ = simple_logging::log_to_file("server.log", LevelFilter::Info);
    let mut connections = vec![PlayerConnection::new(String::from("192.0.0.1"))];

    let mut caluclated_positions : Vec<Circle> = vec![];

    let mut w = World::new();
    // put components you want replicated here
    w.register_type::<EntityComponent>(true);
    w.register_type::<TransformComponent>(true);
    w.register_type::<ChampionComponent>(true);
    w.register_type::<CharacterStateComponent>(true);
    w.register_type::<MinionComponent>(true);
    w.register_type::<TeamComponent>(true);
    w.register_type::<HealthComponent>(true);
    w.register_type::<RadiusComponent>(true);

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
        w.add_component_to_entity(entity, HealthComponent::new(100));
        w.add_component_to_entity(entity, RadiusComponent::new(1.0));
    }
    
    for _i in 0..2 {
        let entity = w.new_entity();
        w.add_component_to_entity(entity, TransformComponent::new(Matrix4::from_translation(Vector3::new(0.0f32, 0.0, 0.0))));
        w.add_component_to_entity(entity, CharacterStateComponent::new());
        w.add_component_to_entity(entity, MinionComponent::new());
        w.add_component_to_entity(entity, TeamComponent::new(1));
        w.add_component_to_entity(entity, HealthComponent::new(100));
        w.add_component_to_entity(entity, RadiusComponent::new(1.0));
    }
    

    loop {

        {
            //Player Connection Input System
            for connection in connections.iter_mut() {
                connection.check_on_player();
            }

            let entity_position_query;
            caluclated_positions = vec![];
            query_3!(EntityComponent, TransformComponent, RadiusComponent, w, entity_position_query);
            for (ec, tc, rc) in entity_position_query {
                caluclated_positions.push(Circle::new(ec.id, tc.position(), rc.radius));
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
                            println!("Processing player input");
                            // we now need to check, did they click on open ground, or a character
                            let what_did_they_click_on = overllaped_entity(Vector2::new(mouse_input.x, mouse_input.y), &caluclated_positions);
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
                                                    println!("Set to attack 1");
                                                }
                                                else {
                                                    println!("Keep attacking exsisting target")
                                                }
                                            },
                                            _ => {
                                                mc.character_state = CharacterState::AutoAttacking(AutoAttackState::new(entity_id, 0));
                                                println!("Set to attack 2");
                                            }
                                        }
                                    }
                                    else {
                                        //If the entity we clicked on was friendly, we simply will walk towards it then
                                        mc.character_state = CharacterState::Moving(Vector3::new(mouse_input.x, mouse_input.y, mouse_input.z));
                                        println!("Set to move 1");
                                    }
                                },
                                None => {
                                    mc.character_state = CharacterState::Moving(Vector3::new(mouse_input.x, mouse_input.y, mouse_input.z));
                                    println!("Set to move 2");
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
                        let difference = location - tc.position();
                        if difference.magnitude() < 1.0 {
                            let move_rune = MoveRune::new(ec.id, difference);
                            rune_system.add_rune(move_rune.into());
                            mc.character_state = CharacterState::Idle;
                        }
                        else {
                            let move_rune = MoveRune::new(ec.id, difference.normalize() * 1.0 * 0.1);
                            rune_system.add_rune(move_rune.into());
                        }
                        


                    },
                    CharacterState::AutoAttacking(mut autoatack_attack) => {
                        autoatack_attack.progress += 1;
                        if autoatack_attack.progress >= 100 {
                            let damage_rune = DamageRune::new(autoatack_attack.target, 10);
                            rune_system.add_rune(damage_rune.into());
                            mc.character_state = CharacterState::Idle;
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
               // rune_system.add_rune(move_rune.into());
            }
        }
        
        

        rune_system.resolve_world_state(&mut w);
        for connection in connections.iter_mut() {
            connection.update_player_with_new_game_state(w.to_byte_array(), w.frame_number);
        }
        sleep(Duration::from_millis(14));
    }
}
