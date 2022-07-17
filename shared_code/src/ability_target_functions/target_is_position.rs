use crate::*;

pub fn target_is_position(ability_target_context: AbilityTargetContext) -> bool { 
    match ability_target_context.ability_target {
        AbilityTarget::Position(_x, _y) => {
            return true;
        },
        _ => {
            return false;
        }
    }
}