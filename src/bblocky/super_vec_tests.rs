use super::super_vec::*;
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
use crate::bblocky::super_any_tests::*;


// #[test]
// fn test_ser_de() {
//     let original = TestStructC{
//         byte_a: 7,
//         byte_b: 2,
//         byte_c: 5
//     };
//     let mut my_vec = SuperVec::new_and_push(original);
//
//     let bytes = bincode::serialize(&my_vec).unwrap();
//     let reser = bincode::deserialize::<SuperVec>(bytes.as_slice()).unwrap();
//
//     assert_eq!(my_vec, reser);
// }
// #[test]
// fn test_from_any() {
//     let original = TestStructC{
//         byte_a: 7,
//         byte_b: 2,
//         byte_c: 5
//     };
//     let super_any = SuperAny::new(original.clone());
//     let my_type = gett::<TestStructC>();
//
//     let mut my_vec = SuperVec::new(my_type);
//
//     my_vec.push_super_any(super_any);
//
//     let bytes = bincode::serialize(&my_vec).unwrap();
//     let reser = bincode::deserialize::<SuperVec>(bytes.as_slice()).unwrap();
//
//     let new_version = reser.get::<TestStructC>(0).unwrap().clone();
//     assert_eq!(original, new_version);
// }
//
// #[test]
// fn test_swap_remove() {
//     let original = TestStructC{
//         byte_a: 7,
//         byte_b: 2,
//         byte_c: 5
//     };
//     let mut my_vec = SuperVec::new_and_push(original.clone());
//
//     let original2 = TestStructC{
//         byte_a: 1,
//         byte_b: 2,
//         byte_c: 6
//     };
//     my_vec.push(original2.clone());
//
//     let original3 = TestStructC{
//         byte_a: 3,
//         byte_b: 8,
//         byte_c: 12
//     };
//     my_vec.push(original3.clone());
//
//     my_vec.swap_remove(0);
//     let first = my_vec.get::<TestStructC>(0).unwrap().clone();
//     assert_eq!(first, original3);
// }
//
//
// #[test]
// fn test_mem_leak() {
//     loop{
//         let original = TestStructB{
//             integer: 0,
//             vec: vec![vec![TestStructA{
//                 integer: 0,
//                 float: 0.0,
//                 vec: vec![]
//             }]],
//             float: 0.0
//         };
//         let mut my_vec = SuperVec::new_and_push(original.clone());
//         my_vec.push(original.clone());
//         my_vec.swap_remove(0);
//     }
// }