use crate::*;
use cgmath::*;
use erased_serde::{Serialize, Serializer};
use serde::{Serialize, Deserialize};
use std::ops::Mul;


use slotmap::{SlotMap, SecondaryMap, DefaultKey};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct PlayerInput {
    input_values: u8
}

impl PlayerInput {
    pub fn new(input_values: u8) -> PlayerInput {
        PlayerInput {
            input_values
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct World<'a> {
    pub frame_number: usize,
    pub entities: SlotMap<DefaultKey, Entity>,
    pub components: HashMap<ComponentType, SecondaryMap<DefaultKey, Box<dyn Component<'a>>>>,
    pub input: u8,
    pub is_moving: bool,
    pub desired_x: u32,
    pub desired_y: u32
}

impl<'a> World<'a> {
    pub fn spawn_entity(&mut self) -> EntityId {
        let entity = Entity{component_mask: 0, id: DefaultKey::default()};
        let id = self.entities.insert(entity);
        self.entities[id].id = id;
        return id;
    }

    pub fn add_component(&mut self, entity: EntityId, component: Box<dyn Component<'a>>) {
        if self.entities.contains_key(entity) {
            if self.components.contains_key(&component.component_type()) == false {
                self.components.insert(component.component_type(), SecondaryMap::new());
            }

            let mut component_list = self.components.get_mut(&component.component_type());
            let actual = component_list.as_mut().unwrap();
            if actual.contains_key(entity) {
                panic!("Tried to two of the same component to a entity");
            }
            //We do it before as component gets moved when we insert
            //TODO: move this up a little early so we can use this as our bail condition
            self.entities[entity].component_mask |= component.component_type() as u64;
            actual.insert(entity, component);
        }
        else {
            panic!("Tried to add a component to an entity that does not exists");
        }
    }

    // This returns those entities that have all of the states Components
    pub fn entities_with_components(&mut self, component_type: Vec<ComponentType>) -> Vec<EntityId> {
        let mask = component_type.iter().fold(0, |sum, i| sum | *i as u64);

        let mut ids = vec![];
        for (id, value) in self.entities.iter() {
            if value.component_mask & mask > 0 {
                //These are all of the entities that have these compoennts
                ids.push(id);
            }
        }

        return ids;
    }

    pub fn tick(&mut self) {
        self.frame_number += 1;
    }

    pub fn post_tick(&mut self) {
      //  self.input = 0;
    }
}