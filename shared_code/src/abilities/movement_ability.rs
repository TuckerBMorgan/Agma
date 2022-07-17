use crate::*;

pub fn basic_movement(ability_context: AbilityContext) {
    let mut position_x = 0;
    let mut position_y = 0;
    {
        if let Some(pc) = ability_context.world.borrow_component_vec::<PositionComponent>().as_mut().unwrap().get_mut(ability_context.casting_character) {
            position_x = pc.as_ref().unwrap().x;
            position_y = pc.as_ref().unwrap().y;
        }
    }

    match ability_context.ability_instance.target {
        AbilityTarget::Character(_id) => {
            //TOOD: Mostly will need to handle logic around moving to a place
        },
        AbilityTarget::Position(desination_x, destination_y) => {
            let mut x_update = desination_x - position_x;
            if x_update != 0 {
                x_update = x_update.signum();
            }
        
            let mut y_update = destination_y - position_y;
            if y_update != 0 {
                y_update = y_update.signum();
            }
            let move_rune = MoveRune::new(ability_context.casting_character, x_update, y_update);
            ability_context.rune_system.add_rune(move_rune.into());
        }
        AbilityTarget::None => {
            panic!("Unable to move to no where");
        }
    }
}