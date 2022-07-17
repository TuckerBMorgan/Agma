use crate::*;

pub fn autoattack(ability_context: AbilityContext) {
    match ability_context.ability_instance.target {
        AbilityTarget::Character(target_id) => {
            let damage_rune = DamageRune::new(ability_context.casting_character, target_id, 10);
            ability_context.rune_system.add_rune(damage_rune.into());        
        },
        _ => {
            
        }
    }
}