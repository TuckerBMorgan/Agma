use std::collections::HashMap;
use crate::*;

pub type AbilityActionFunctionIndex = usize;
pub type AbilityTargetValidatorFunctionIndex = usize;

pub enum AbilityTarget {
    None,
    Character(usize),
    Position(i64, i64)
}


pub struct AbilityTargetContext<'a> {
    pub world: &'a World,
    pub casting_character: usize,
    pub ability_instance: &'a AbilityInstance,
    pub ability_target: AbilityTarget
}

impl<'a> AbilityTargetContext<'_> {
    pub fn new(world: &'a World, casting_character: usize, ability_instance: &'a AbilityInstance, ability_target: AbilityTarget) -> AbilityTargetContext<'a> {
        AbilityTargetContext {
            world,
            casting_character,
            ability_instance,
            ability_target
        }
    }   
}

pub struct AbilityTargetFunctionLibrary {
    pub ability_count: AbilityTargetValidatorFunctionIndex,
    pub functions: HashMap<usize, fn(AbilityTargetContext) -> bool>
}


impl AbilityTargetFunctionLibrary {
    
    pub fn new() -> AbilityTargetFunctionLibrary {
        AbilityTargetFunctionLibrary {
            ability_count: 0,
            functions: HashMap::new()
        }
    }

    pub fn add_function(&mut self, function: fn(AbilityTargetContext) -> bool) -> AbilityTargetValidatorFunctionIndex { 
        self.ability_count += 1;
        self.functions.insert(self.ability_count, function);
        return self.ability_count;
    }

    pub fn call_target_validator_function(&self, ability_action_index: AbilityTargetValidatorFunctionIndex, context: AbilityTargetContext) -> bool {
        self.functions[&ability_action_index](context)
    }
}


pub struct AbilityContext<'a> {
    pub world: &'a mut World,
    pub casting_character: usize,
    pub rune_system: &'a mut RuneSystem,
    pub ability_instance: &'a mut AbilityInstance
}

impl<'a> AbilityContext<'_> {
    pub fn new(world: &'a mut World, casting_character: usize, rune_system: &'a mut RuneSystem, ability_instance: &'a mut AbilityInstance) -> AbilityContext<'a> {
        AbilityContext {
            world,
            casting_character,
            rune_system,
            ability_instance
        }
    }
}

pub struct AbilityFunctionLibrary {
    pub ability_count: AbilityActionFunctionIndex,
    pub functions: HashMap<usize, fn(AbilityContext) -> ()>
}

impl AbilityFunctionLibrary {
    pub fn new() -> AbilityFunctionLibrary {
        AbilityFunctionLibrary {
            ability_count: 0,
            functions: HashMap::new()
        }
    }

    pub fn add_function(&mut self, function: fn(AbilityContext) -> ()) -> AbilityActionFunctionIndex { 
        self.ability_count += 1;
        self.functions.insert(self.ability_count, function);
        return self.ability_count;
    }

    pub fn call_action_function(&self, ability_action_index: AbilityActionFunctionIndex, context: AbilityContext) {
        self.functions[&ability_action_index](context);
    }
}

#[derive(Eq, PartialEq)]
pub enum AbilityState {
    Ready,
    Channeling,
    Recovering
}

pub struct AbilityInstance {
    pub target_validator: AbilityTargetValidatorFunctionIndex,
    pub ability_state: AbilityState,
    pub channel_time: usize,
    pub current_channel_time: usize,
    pub action: AbilityActionFunctionIndex,
    pub recovery_time: usize,
    pub current_recovery_time: usize,
    pub target: AbilityTarget,
    pub range: usize
}

impl AbilityInstance {
    pub fn new(channel_time: usize, recovery_time: usize, action: AbilityActionFunctionIndex, target_validator: AbilityTargetValidatorFunctionIndex, range: usize) -> AbilityInstance {
        AbilityInstance {
            target_validator,
            ability_state: AbilityState::Ready,
            channel_time,
            current_channel_time: 0,
            action,
            recovery_time,
            current_recovery_time: 0,
            target: AbilityTarget::None,
            range
        }
    }

    pub fn start_with_target(&mut self, target: AbilityTarget) {
        self.target = target;
        self.ability_state = AbilityState::Channeling;
    }

    pub fn advance(&mut self) -> bool {
        match self.ability_state {
            AbilityState::Channeling => {
                self.current_channel_time += 1;
                if self.current_channel_time >= self.channel_time {
                    self.current_channel_time = 0;
                    self.ability_state = AbilityState::Recovering;
                    return true;
                }
                return false;
            },
            AbilityState::Recovering => {
                self.current_recovery_time += 1;
                if self.current_recovery_time >= self.channel_time {
                    self.current_recovery_time = 0;
                    self.ability_state = AbilityState::Ready;
                }
                return false;
            },
            _ => {
                return false;
            }
        }
    }

    pub fn reset(&mut self) {
        self.current_channel_time = 0;
        self.current_recovery_time = 0;
        self.ability_state = AbilityState::Ready;
    }

}

pub type AbilityInstanceId = usize;
pub const NULL_ABILITY_VALUE : usize = 0;

pub struct AbilityInstanceLibrary {
    pub ability_count: usize,
    pub ability_instances: HashMap<AbilityInstanceId, AbilityInstance>,
}

impl AbilityInstanceLibrary {
    pub fn new() -> AbilityInstanceLibrary {
        AbilityInstanceLibrary {
            ability_count: 0,
            ability_instances: HashMap::new()
        }
    }

    pub fn add_new_ability_instance(&mut self, ability_instance: AbilityInstance) -> AbilityInstanceId {
        self.ability_count += 1;
        self.ability_instances.insert(self.ability_count, ability_instance);
        return self.ability_count;
    }

    pub fn checkout_ability_instance(&mut self, id: AbilityInstanceId) -> Option<&mut AbilityInstance> {
        return self.ability_instances.get_mut(&id);
    }
}

// Since we preload these functions we just need to 
pub const BASIC_MOVEMENT_ABILITY_FUNCTION_ID : AbilityActionFunctionIndex = 1;
pub const AUTOATTACK_ABILITY_FUNCTION_ID : AbilityActionFunctionIndex = 2;

pub fn load_basic_functions() -> AbilityFunctionLibrary {
    let mut ability_library = AbilityFunctionLibrary::new();
    let _movement_ability_id = ability_library.add_function(basic_movement);
    let _autoattack = ability_library.add_function(autoattack);
    return ability_library;
}