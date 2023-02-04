use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::*;
use serde::de::Visitor;
use serde::ser::SerializeStruct;

use crate::bblocky::comp_registration::EcsConfig;
use crate::comp_store::*;
use crate::GlobalEntityID;
use crate::pending_entity::PendingEntity;

#[derive(Debug, Serialize, Deserialize)]
pub struct VelvetEcs<S> {
    #[serde(skip)]
    systems: Vec<System<S>>,
    pub c: CompStorage,
}
impl<S> VelvetEcs<S> {
    pub fn post_deserialize(&mut self, config: EcsConfig<S>){
        self.c.post_deserialize(&config.functions);
        self.systems = config.systems;
    }
    pub fn new(config: EcsConfig<S>) -> Self{
        Self{
            systems: config.systems,
            c: CompStorage::new(config.functions),
        }
    }
    pub fn set_systems(&mut self, systems: Vec<System<S>>){
        self.systems = systems;
    }
    pub fn sim_systems(&mut self, stat: &S){
        self.c.flush_ent_changes();
        for system in &self.systems{
            (system.run)(&mut self.c, stat);
            self.c.flush_ent_changes();
        }
    }
}

impl<S> Hash for VelvetEcs<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.c.hash(state);
    }
}
#[derive(Clone)]
pub struct System<S>{
    pub run: fn(&mut CompStorage, &S),
    pub name: &'static str,
}
impl<S> Debug for System<S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("System").field("Name: ", &self.name).finish()
    }
}
