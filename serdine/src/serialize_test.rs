use crate as serdine;
use crate::Serialize as DeserializeDisambiguate;
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
    pub my_bool: bool,
    #[serialize = "serialize_vec"]
    pub my_vec: Vec<u8>,
}

fn serialize_vec<W: std::io::Write>(vec: &Vec<u8>, mut w: W) -> Result<(), std::io::Error> {
    for instance in vec {
        instance.serialize(&mut w)?;
    }
    Ok(())
}

#[test]
fn test_serialize_named_fields_struct() {
    let instance = MyNamedFieldsStruct {
        my_i16: 0x80,
        my_u32: 0xCAFEBABE,
        my_f32: 1004.981_f32,
        my_f64: 10.04981_f64,
        my_arr: [0x0100, 0x0302],
        my_bool: true,
        my_vec: vec![4, 5, 6],
    };

    let mut serialized_instance = Vec::new();

    instance.serialize(&mut serialized_instance).unwrap();

    #[rustfmt::skip]
    let expected_bytes: &[u8] = &[
        0x80, 0x00,
        0xBE, 0xBA, 0xFE, 0xCA,
        0xC9, 0x3E, 0x7B, 0x44,
        0x0C, 0x07, 0x42, 0xB2, 0x80, 0x19, 0x24, 0x40,
        0x00, 0x01, 0x02, 0x03,
        0x01,
        0x04, 0x05, 0x06
    ];

    assert_eq!(expected_bytes, serialized_instance);
}

// ////////////////////////////////////////////////////////////////////////////////
// ENUMS
// ////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize)]
#[repr(u16)]
enum MyEnum {
    VarA = 0,
    VarB = 1,
    VarC = 65534,
}

#[test]
fn test_serialize_enum() {
    let mut serialized_instance = Vec::new();

    MyEnum::VarA.serialize(&mut serialized_instance).unwrap();
    MyEnum::VarC.serialize(&mut serialized_instance).unwrap();
    MyEnum::VarB.serialize(&mut serialized_instance).unwrap();

    #[rustfmt::skip]
    let expected_bytes: &[u8] = &[
        0x00, 0x00,
        0xFE, 0xFF,
        0x01, 0x00,
    ];

    assert_eq!(expected_bytes, serialized_instance);
}
