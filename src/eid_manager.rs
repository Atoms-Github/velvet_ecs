use std::collections::BTreeSet;
use serde::*;
use std::convert::TryInto;
use crate::comp_store::{InternalEntity};

use serde_big_array::*;
use std::fmt::{Debug, Formatter};

pub type GlobalEntityID = usize;
// pub const MAX_ENTITIES :usize = 512; // Debug mode.
pub const MAX_ENTITIES :usize = 4096; // Can work on release mode.



// big_array! { BigArray; }


#[derive(Clone, Serialize, Deserialize, Hash)]
pub struct GlorifiedHashMap {
    #[serde(with = "BigArray")]
    alive: [bool; MAX_ENTITIES],
    #[serde(with = "BigArray")]
    entity_ids: [GlobalEntityID; MAX_ENTITIES],
    #[serde(with = "BigArray")]
    internal_details: [InternalEntity; MAX_ENTITIES],
}
impl Debug for GlorifiedHashMap{
    fn fmt(&self, f: &mut Formatter<'_>) -> __private::fmt::Result {
        f.debug_struct("GlorifiedHashMap").finish()
    }
}
impl Default for GlorifiedHashMap{
    fn default() -> Self {
        Self::new()
    }
}

impl GlorifiedHashMap {
    pub fn new() -> Self{
        let mut entity_ids = vec![];
        let mut internal_details = vec![];
        for i in 0..MAX_ENTITIES{
            entity_ids.push(i);
            internal_details.push(InternalEntity::default());
        }
        Self{
            alive: [false; MAX_ENTITIES],
            entity_ids: entity_ids.as_slice().try_into().unwrap(),
            internal_details: internal_details.try_into()
                .unwrap_or_else(|v: Vec<InternalEntity>| panic!("Expected a Vec of length {} but it was {}", MAX_ENTITIES, v.len()))
        }
    }
    pub fn create_entity(&mut self, mut internal_entity: InternalEntity) -> GlobalEntityID{
        for index in 0..MAX_ENTITIES{
            if !self.alive[index]{
                self.alive[index] = true;
                let new_global_id = self.entity_ids[index];
                self.entity_ids[index] = new_global_id;
                internal_entity.global_id = new_global_id;
                self.internal_details[index] = internal_entity;
                return new_global_id;
            }
        }
        panic!("Exceeded entity storage capacity! Increase MAX_ENTITIES.");
    }
    pub fn delete(&mut self, entity_id: GlobalEntityID) -> Option<InternalEntity>{
        let index = entity_id % MAX_ENTITIES;
        // If an entity lives in the spot.
        if self.alive[index]{
            // If the correct generation.
            if self.entity_ids[index] == entity_id {
                self.alive[index] = false;
                self.entity_ids[index] += MAX_ENTITIES;

                // TODO: Make this work? So it actually pops it. return Some(self.internal_details[index]);
            }
        }
        return None;
    }
    pub fn get_mut(&mut self, query_id: GlobalEntityID) -> Option<&mut InternalEntity>{
        let index = query_id % MAX_ENTITIES;
        // If an entity lives in the spot.
        if self.alive[index]{
            // If the correct generation.
            if self.entity_ids[index] == query_id{
                return Some(&mut self.internal_details[index]);
            }
        }
        return None;
    }
    pub fn get(&self, query_id: GlobalEntityID) -> Option<&InternalEntity>{
        let index = query_id % MAX_ENTITIES;
        // If an entity lives in the spot.
        if self.alive[index]{
            // If the correct generation.
            if self.entity_ids[index] == query_id{
                return Some(&self.internal_details[index]);
            }
        }
        return None;
    }
}


#[cfg(test)]
mod tests {
    use crate::eid_manager::*;

    #[test]
    fn basic() {
        // let comp1 = InternalEntity{
        //     global_id: 3,
        //     composition_id: 2,
        //     internal_index: 1
        // };
        // let comp2 = InternalEntity{
        //     global_id: 5,
        //     composition_id: 4,
        //     internal_index: 3
        // };
        // let mut storage = GlorifiedHashMap::new();
        // let id1 = storage.create_entity(comp1);
        // let id2 = storage.create_entity(comp2);
        // storage.delete(id1);
        // assert_eq!(comp2, *storage.get(id2).unwrap());

    }
}

