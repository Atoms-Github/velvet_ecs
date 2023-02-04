use super::super_any_tests::*;
use crate::bblocky::*;
use crate::comp_store::*;


use std::any::{TypeId, Any};
use std::collections::HashMap;
use serde::*;

use serde::de::DeserializeOwned;
use crate::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;
use std::fmt::{Write, Debug, Formatter};
use std::fmt;
use std::mem::MaybeUninit;
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;
use crate::velvet_ecs::System;
use crate::utils::{TypeIdNum, gett, crack_type_id};



pub struct EcsConfig<S>{
    pub functions: FunctionMap,
    pub systems: Vec<System<S>>,
}

#[derive(Clone, Default)]
pub struct FunctionMap{
    map: HashMap<TypeIdNum, SuperbFunctions>,
}
impl Hash for FunctionMap{
    fn hash<H: Hasher>(&self, state: &mut H) {
    }
}
impl Debug for FunctionMap{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("A FunctionMap")
    }
}
struct MyData{
    numa: u8,
    numb: u8,
    numc: u8,
}
impl FunctionMap{
    pub fn register_type<T : 'static + Serialize + Clone + DeserializeOwned + Send + Debug>(&mut self){

        self.map.insert(gett::<T>(), get_functions::<T>());
    }
    pub fn get_from_type_id(&self, type_id: TypeId) -> &SuperbFunctions {
        return self.get(crack_type_id(type_id));
    }
    pub fn get(&self, type_id_num: TypeIdNum) -> &SuperbFunctions {
        return self.map.get(&type_id_num).unwrap_or_else(|| panic!("Type wasn't registered!: {}", type_id_num));

    }
}
pub fn get_functions<T : 'static + Serialize + Clone + DeserializeOwned + Send + Debug>() -> SuperbFunctions{
    let size = std::mem::size_of::<T>();

    assert!(size > 0, "Components with size of 0 are disallowed.");
    SuperbFunctions {
        do_clone: |item| {
            let casted = (*item).downcast_ref::<T>().unwrap();
            Box::new(casted.clone())
        },
        ser: |item| {
            let casted = (*item).downcast_ref::<T>().unwrap();
            return bincode::serialize(casted).unwrap();
        },
        deser: |bytes| {
            let item = bincode::deserialize::<T>(bytes).unwrap();
            return Box::new(item);
        },
        meme_ser: |item| {
            let as_type :&T = unsafe{ crate::utils::u8_slice_to_ref(item)};
            return bincode::serialize(as_type).unwrap();
        },
        meme_deser_and_forget: |serialized_bytes| {
            let item : T = bincode::deserialize::<T>(serialized_bytes).unwrap();
            let my_ref = unsafe{ crate::utils::struct_as_u8_slice(&item)};
            let to_return = my_ref.to_vec();
            std::mem::forget(item); // TODO: Confirm this.
            return to_return;
        },
        meme_clone_and_forget: |original_bytes|{
            let as_type :&T = unsafe{ crate::utils::u8_slice_to_ref(original_bytes)};
            let cloned = as_type.clone();
            let back_to_bytes = unsafe{ crate::utils::struct_as_u8_slice(&cloned)}.to_vec();
            std::mem::forget(cloned);
            return back_to_bytes;
        },
        deallocate_refed_mem: |bytes|{
            // What we want to do:
            // 1. Turn bytes into an object.
            // 2. Run forget or drop or similar (the one that keeps the object but drops refed mem).
            // We should be safe, as we're unable to modify source bytes. Hmm. Since we're going unsafe, maybe not.
            let as_type :&T = unsafe{ crate::utils::u8_slice_to_ref(bytes)};
            unsafe{
                let mut e = MaybeUninit::<T>::zeroed().assume_init();
                let target_bytes = crate::utils::struct_as_u8_slice_mut(&mut e);
                // Load e up with values.
                target_bytes.clone_from_slice(bytes);
                // Now drop e, deleting all refed values.
                std::mem::drop(e);
            }
        },
        item_size: size,
        debug_name: std::any::type_name::<T>().to_string(),
        debug_fmt: |item|{
            let as_type :&T = unsafe{ crate::utils::u8_slice_to_ref(item)};
            return format!("{:?}", as_type);
        }
    }
}
#[derive(Clone)]
pub struct SuperbFunctions {
    pub do_clone: fn(&Box<dyn Any + Send>) -> Box<dyn Any + Send>,
    pub ser: fn(&Box<dyn Any + Send>) -> Vec<u8>,
    pub deser: fn(&Vec<u8>) -> Box<dyn Any + Send>,

    pub meme_ser: fn(&[u8]) -> Vec<u8>,
    pub meme_deser_and_forget: fn(&Vec<u8>) -> Vec<u8>,
    pub meme_clone_and_forget: fn(&[u8]) -> Vec<u8>,

    pub deallocate_refed_mem: fn(&[u8]),

    pub item_size: usize,
    pub debug_name: String,
    pub debug_fmt: fn(&[u8]) -> String,
}
impl PartialEq for SuperbFunctions{
    fn eq(&self, other: &Self) -> bool {
        return self.debug_name.eq(&other.debug_name)
    }
}