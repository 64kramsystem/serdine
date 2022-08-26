use crate as serdine;
use crate::Deserialize as DeserializeDisambiguate;
use serdine_derive::Deserialize;

// ////////////////////////////////////////////////////////////////////////////////
// STRUCT WITH NAMED FIELDS
// ////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
pub struct MyNamedFieldsStruct {
    pub my_i16: i16,
    pub my_u32: u32,
    pub my_f32: f32,
    pub my_f64: f64,
    pub my_arr: [u16; 2],
    pub my_bool: bool,
    #[deserialize = "deserialize_vec"]
    pub my_vec: Vec<u8>,
}

fn deserialize_vec<R: std::io::Read>(mut r: R) -> Vec<u8> {
    let mut buffer = Vec::new();
    r.read_to_end(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_deserialize_named_fields_struct() {
    #[rustfmt::skip]
    let serialized_bytes: &[u8] = &[
        0x80, 0x00,
        0xBE, 0xBA, 0xFE, 0xCA,
        0xC9, 0x3E, 0x7B, 0x44,
        0x0C, 0x07, 0x42, 0xB2, 0x80, 0x19, 0x24, 0x40,
        0x00, 0x01, 0x02, 0x03,
        0xCA,
        0x04, 0x05, 0x06
    ];

    let instance = MyNamedFieldsStruct::deserialize(serialized_bytes);

    assert_eq!(0x80, instance.my_i16);
    assert_eq!(0xCAFEBABE, instance.my_u32);
    assert_eq!(1004.981_f32, instance.my_f32);
    assert_eq!(10.04981_f64, instance.my_f64);
    assert_eq!([0x0100, 0x0302], instance.my_arr);
    assert!(instance.my_bool);
    assert_eq!(vec![4, 5, 6], instance.my_vec);
}

// ////////////////////////////////////////////////////////////////////////////////
// ENUMS
// ////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, PartialEq)]
#[repr(u16)]
enum MyEnum {
    VarA = 0,
    VarB = 1,
    VarC = 65534,
}

#[test]
fn test_deserialize_enum() {
    #[rustfmt::skip]
    let serialized_bytes: &[u8] = &[
        0x00, 0x00,
        0xFE, 0xFF,
        0x01, 0x00,
    ];

    let mut reader = serialized_bytes;

    assert_eq!(MyEnum::VarA, serdine::Deserialize::deserialize(&mut reader));
    assert_eq!(MyEnum::VarC, serdine::Deserialize::deserialize(&mut reader));
    assert_eq!(MyEnum::VarB, serdine::Deserialize::deserialize(&mut reader));
}
