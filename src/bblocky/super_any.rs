use std::any::{TypeId, Any};
use std::collections::HashMap;
use serde::*;

use serde::de::DeserializeOwned;
use crate::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;
use std::fmt::{Write, Debug};
use std::fmt;
use super::comp_registration::*;
use crate::bblocky::super_vec::SuperVec;
use serde::*;

use crate::comp_store::*;
use super::comp_registration::*;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuperAny {
    pub list: SuperVec,
}
impl SuperAny {
    pub fn new<T : 'static + Serialize + Clone + DeserializeOwned + Send + Debug>(item: T) -> Self{
        Self{
            list: SuperVec::new_and_push(item),
        }
    }
    pub fn get<T : 'static + Serialize + Clone + DeserializeOwned + Send + Debug>(&self) -> &T{
        return self.list.get::<T>(0).unwrap();
    }
    pub fn get_mut<T : 'static + Serialize + Clone + DeserializeOwned + Send + Debug>(&mut self) -> &mut T{
        return self.list.get_mut::<T>(0).unwrap();
    }
}