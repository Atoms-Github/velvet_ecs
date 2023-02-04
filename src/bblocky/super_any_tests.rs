use super::super_any::*;
use std::any::{TypeId, Any};
use std::collections::HashMap;
use serde::*;

use serde::de::DeserializeOwned;
use crate::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;
use std::fmt::Write;
use std::fmt;
use crate::bblocky::super_any::SuperAny;


#[test]
fn test_ser_de() {
    let original = TestStructB{
        integer: 3,
        vec: vec![vec![], vec![TestStructA{
            integer: 0,
            float: 3.7,
            vec: vec![8,5]
        }]],
        float: 100.2
    };
    let strb = SuperAny::new(original.clone());
    let bytes = bincode::serialize(&strb).unwrap();
    let reser = bincode::deserialize::<SuperAny>(bytes.as_slice()).unwrap();

    let new_version = reser.get::<TestStructB>().clone();
    assert_eq!(original, new_version);
}
// #[test]
// fn test_super_any_ser_de_mem_leak() {
//     loop{
//         let original = TestStructB{
//             integer: 3,
//             vec: vec![vec![], vec![TestStructA{
//                 integer: 0,
//                 float: 3.7,
//                 vec: vec![8,5]
//             }]],
//             float: 100.2
//         };
//         let strb = SuperAny::new(original.clone());
//         let bytes = bincode::serialize(&strb).unwrap();
//         let reser = bincode::deserialize::<SuperAny>(bytes.as_slice()).unwrap();
//
//         let new_version = reser.get::<TestStructB>().clone();
//         assert_eq!(original, new_version);
//         println!("{:?}", new_version);
//     }
// }

#[test]
fn test_clone() {
    let original = TestStructB{
        integer: 3,
        vec: vec![vec![], vec![TestStructA{
            integer: 0,
            float: 3.7,
            vec: vec![8,5]
        }]],
        float: 100.2
    };
    let super_any = SuperAny::new(original.clone());
    assert_eq!(*super_any.clone().get::<TestStructB>(), original);
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TestStructA{
    pub integer: u32,
    pub float: f32,
    pub vec: Vec<i32>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TestStructB{
    pub integer: u32,
    pub vec: Vec<Vec<TestStructA>>,
    pub float: f32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TestStructC{
    pub byte_a: u8,
    pub byte_b: u8,
    pub byte_c: u8,
}
