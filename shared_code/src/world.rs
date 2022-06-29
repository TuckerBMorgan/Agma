

use std::cell::{RefCell, RefMut};
use bincode::{config, Decode, Encode};
use crate::*;

use cgmath::*;
use crate::components::*;

/*
pub trait Resource {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: Encode + Decode + Copy + Clone + 'static> Resource for RefCell<Vec<Option<T>>> {
    // Same as before
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    // Same as before
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}
*/
pub trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
    fn to_byte_array(&mut self) -> Vec<u8>;
}


impl<T: Encode + Decode + Copy + Clone + 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    // Same as before
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    // Same as before
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        // `&mut self` already guarantees we have
        // exclusive access to self so can use `get_mut` here
        // which avoids any runtime checks.
        self.get_mut().push(None)
    }

    // Convert a single array of components into a byte array
    fn to_byte_array(&mut self) -> Vec<u8> {
        let config = config::standard();
        let mut big_chunky_array = vec![];
        for element in self.get_mut() {
            match element {
                Some(t) => {
                    let encoded: Vec<u8> = bincode::encode_to_vec(*t, config).unwrap();

                    big_chunky_array.extend(encoded.len().to_le_bytes());

                    big_chunky_array.extend(encoded);
                },
                None => {
                    big_chunky_array.extend([0, 0, 0, 0, 0, 0, 0, 0]);
                }
            }
        }
        big_chunky_array
    }
}

type Builder = for<'r, 's> fn(&'r mut World, &'s [u8], usize);

/// The World for the ECS
pub struct World {
    /// What is the current simulation frame
    pub frame_number: usize,
    /// How many entities we have in the world
    entities_count: usize,
    /// vectors of each component
    component_vecs: Vec<Box<dyn ComponentVec>>,
    /// A mapping for if we should be replicating a particular component
    should_replicate: Vec<bool>,

    builder_functions: Vec<Builder>,

   // resources: Vec<Box<dyn Resource>>
}

impl World {
    pub fn new() -> Self {
        Self {
            frame_number: 0,
            entities_count: 0,
            component_vecs: vec![],
            should_replicate: vec![],
            builder_functions: vec![]
        }
    }

    /// given a complete world state rebuild all of the component vectors for a new world 
    pub fn new_from_byte_array(bytes: Vec<u8>) -> World {
        let mut world = World::new();
        world.entities_count += 1;
        world.rebuild_world(bytes);
        return world;
    }

    //We have to build the world component vec arrays outselves, as the ComponentVec type is
    //agnostic to it's type
    pub fn rebuild_world(&mut self, bytes: Vec<u8>) {
        let mut current_index = 0;
        loop {
            let mut size_bytes = [0, 0, 0, 0, 0, 0, 0, 0];
            for i in 0..8 {
                //We need to offset by 1 as current_index is the a u8 representing which 
                //type of component we are decoding
                size_bytes[i] = bytes[current_index + 1 + i];
            }
            
            let number_of_bytes = usize::from_le_bytes(size_bytes);

            let data_start = current_index + 9;
            let end_position = data_start + number_of_bytes;
            let range = data_start..end_position;
            if bytes[range.clone()].len() > 0 {
                self.builder_functions[bytes[current_index] as usize](self, &bytes[range], bytes[current_index] as usize);
            }
            current_index = end_position;
            if current_index >= bytes.len() {
                return;
            }
        }
    }

    fn rebuild_component_array<ComponentType: Encode + Decode + Copy + Clone + 'static>(&mut self, byte_array: &[u8], index: usize) {
        let test = self.decode_component_vector_from_byte_array::<ComponentType>(&byte_array);
        //We are setting the number of entities we have
        if index == 0 {
            self.entities_count = test.len();
        }
        self.component_vecs[index] = Box::new(RefCell::new(test));
    }

    //TODO: rip this out, as it will clash with those things that use TeamComponent in a system
    pub fn are_friends(&self, entity_a: usize, entity_b: usize) -> bool {
        let team_components = self.borrow_component_vec::<TeamComponent>().unwrap();
        let a = team_components[entity_a];
        let b = team_components[entity_b];
        let a = a.unwrap();
        let b = b.unwrap();

        return a.team == b.team;
    }

    pub fn entity_at_point(&self, location: Vector2<i64>) -> Option<usize> {
        //There is no "height" in the game so we force callers of this function to call it
        //with a vector2, we store everything as Vector3 though, so we reconvert it
        let location = Vector2::new(location.x, location.y);
        // 1. Create a list of circle that represent a character and their radius
        // 2. Compre the point where the player click against all of them
        // 3. If the player intersected with nothing, then find the location on the map 
        //      cloest to where the player clicked and return that
        // 4. if the player clicked an entity, return that entities ids
        // TODO: create a "RadiusComponent" to help speed up some of this work
        // TODO: build some form of BST to help speed this up even more, this could get REALLLLLY BAD
        let test_radius = 100 as i64;
        let entity_transform_query;
        query_2!(PositionComponent, EntityComponent, self, entity_transform_query);
        let mut lowest_distance = std::i64::MAX;
        let mut the_entity_id = 0;

        for (transform, entity) in entity_transform_query {
            let distance = (Vector2::new(transform.x, transform.y) - location).magnitude2();
            let distance = distance as i64;
            if distance <= test_radius && distance < lowest_distance {
                lowest_distance = distance;
                the_entity_id = entity.id;
            }
        }

        if lowest_distance != std::i64::MAX {
            return Some(the_entity_id);
        }
        return None;
    }

    /// allocate the space in the component_vec for a particular type of Component and then register it into the
    /// replication sytesm as well
    pub fn register_type<ComponentType: Encode + Decode + Copy + Clone + 'static>(&mut self, should_replicate: bool) {
        if should_replicate == true {
            self.builder_functions.push(
                World::rebuild_component_array::<ComponentType>
            );
        }

        let mut none : Vec<Option<ComponentType>> = vec![];
        
        for _en in 0..self.entities_count {
            none.push(None);
        }

        self.component_vecs.push(Box::new(RefCell::new(none)));
        self.should_replicate.push(should_replicate);
    }

    /// Allocates a new entity into the world, allocating space in the vectors for it
    /// and returning an id that can be used to look it up later
    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vecs in self.component_vecs.iter_mut() {
            component_vecs.push_none();
        }
        
        // The entity component is used to look up the Id for the 
        self.add_component_to_entity(entity_id, EntityComponent::new(entity_id));
        self.entities_count += 1;
        entity_id
    }

    /// given a byte vector recreate the vector of a particular kind of component
    pub fn decode_component_vector_from_byte_array<ComponentType: Encode + Decode + Copy + Clone + 'static>(&self, byte_array: &[u8]) -> Vec<Option<ComponentType>> {
        let mut new_component_vecter = vec![];
        let mut current_index = 0;
        loop {
            let mut size_bytes = [0, 0, 0, 0, 0, 0, 0, 0];
            for i in 0..8 {
                //We offset by 1 as current_index is 
                size_bytes[i] = byte_array[current_index + i];
            }

            let data_size = usize::from_le_bytes(size_bytes);
            if data_size == 0 {
                new_component_vecter.push(None);
                current_index += 8;
            }
            else {
                let data_start = current_index + 8;
                let config = config::standard();
                let (ct, _len) : (ComponentType, usize) = bincode::decode_from_slice(&byte_array[data_start..data_start + data_size], config).unwrap();
                new_component_vecter.push(Some(ct));
                current_index = current_index + data_size + 8;
            }
            if current_index >= byte_array.len() {
                break;
            }
        }
        return new_component_vecter;
    }

    /// add a component to an entity
    pub fn add_component_to_entity<ComponentType: Encode + Decode + Copy + Clone + 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {

        for component_vec in self.component_vecs.iter_mut() {
            // The `downcast_mut` type here is changed to `RefCell<Vec<Option<ComponentType>>`
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                let component_vec = component_vec.get_mut();
                if component_vec.len() <  self.entities_count {
                    let missing_entities = self.entities_count - component_vec.len();
                    for _ in 0..missing_entities {
                        component_vec.push(None);
                    }
                }
                // add a `get_mut` here. Once again `get_mut` bypasses
                // `RefCell`'s runtime checks if accessing through a `&mut` reference.
                component_vec[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);

        // Here we create a `RefCell` before inserting into `component_vecs`
        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    /// Get all components of a particular type
    pub fn borrow_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                // Here we use `borrow_mut`. 
                // If this `RefCell` is already borrowed from this will panic.
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    /// Convert the world to a byte array
    pub fn to_byte_array(&mut self) -> Vec<u8> {
        let mut final_big_chonky_array = vec![];
        for (index, array) in self.component_vecs.iter_mut().enumerate() {
            if index >= 255 {
                panic!("We have too many component types to fit into a u8");
            }
            final_big_chonky_array.push(index as u8);
            if self.should_replicate[index] {
                let byte_array = array.to_byte_array();
                //Size header
                final_big_chonky_array.extend(byte_array.len().to_le_bytes());
                //Actual data
                final_big_chonky_array.extend(byte_array);
            }
            else {
                // we don't want to have this component be replicated, but we need
                // it in place on the client so component ids still match
                // so simply push 0
                // TODO: find a way to push all non replicated components at the end of the
                // component vec array
                final_big_chonky_array.extend([0, 0, 0, 0, 0, 0, 0, 0]);
            }
        }
        return final_big_chonky_array;
    }
}