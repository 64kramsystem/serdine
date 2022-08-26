use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::Write,
};

use crate::{self as serdine, Serialize};
use serdine_derive::Serialize;

// ////////////////////////////////////////////////////////////////////////////////
// STRUCT WITH NAMED FIELDS
// ////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize)]
pub struct MyNamedFieldsStruct {
    pub my_i16: i16,
    pub my_u32: u32,
    pub my_f32: f32,
    pub my_f64: f64,
    pub my_arr: [u16; 2],
    #[serialize = "serialize_vec"]
    pub my_vec: Vec<u8>,
}

fn serialize_vec<W: Write>(vec: &Vec<u8>, mut w: W) {
    for instance in vec {
        instance.serialize(&mut w);
    }
}

#[test]
fn test_serialize_named_fields_struct() {
    let instance = MyNamedFieldsStruct {
        my_i16: 0x80,
        my_u32: 0xCAFEBABE,
        my_f32: 1004.981_f32,
        my_f64: 10.04981_f64,
        my_arr: [0x0100, 0x0302],
        my_vec: vec![4, 5, 6],
    };

    let mut serialized_instance = Vec::new();

    instance.serialize(&mut serialized_instance);

    let mut hash = DefaultHasher::new();
    serialized_instance.hash(&mut hash);

    assert_eq!(0xF414562B918E708B, hash.finish());
}
