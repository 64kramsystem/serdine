![Logo](/images/serdine.jpg?raw=true)

# Serdine

Serdine is a tiny serialization library for de/serializing instances in a binary, serial format, focused on ease of use.

This is convenient for example, when interfacing with data files belonging to C programs, where the files are often the image of the in-memory instances.

## Status

This library is currently used by another project of mine ([Catacomb II-64k](https://github.com/64kramsystem/catacomb_ii-64k)), so I don't need to add any new feature, however, if anybody happened to find it useful, I can easily extend it.

Some ideas:

- [ ] Add support for tuple structs support
- [ ] Add support for unions
- [ ] Add support for packed structs
- [ ] Add support for big endian (as feature)
- [ ] Make `no_std`
- [ ] Automatically implement `Vec<_>` when it's the last field

## Design and examples

The crate provides de/serialization implementations for the numeric primites and arrays, in little endian format; any field, particularly those of user-defined types, can use a custom de/serialization function.
De/serialization is recursive, so any type can be nested.

The de/serialization traits are extremely simple:

```rs
pub trait Serialize {
    fn serialize<W: Write>(&self, w: W) -> Result<(), std::io::Error>;
}

pub trait Deserialize {
    fn deserialize<R: Read>(r: R) -> Result<Self, std::io::Error>;
}
```

The following are examples of de/serialization:

```rs
use serdine_derive::Deserialize;

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

fn deserialize_vec<R: std::io::Read>(mut r: R) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = Vec::new();
    r.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn deserialize_named_fields_struct() {
    let serialized_bytes: &[u8] = &[
        0x80, 0x00,
        0xBE, 0xBA, 0xFE, 0xCA,
        0xC9, 0x3E, 0x7B, 0x44,
        0x0C, 0x07, 0x42, 0xB2, 0x80, 0x19, 0x24, 0x40,
        0x00, 0x01, 0x02, 0x03,
        0xCA,
        0x04, 0x05, 0x06
    ];

    let instance = MyNamedFieldsStruct::deserialize(serialized_bytes).unwrap();

    assert_eq!(0x80,                        instance.my_i16);
    assert_eq!(0xCAFEBABE,          instance.my_u32);
    assert_eq!(1004.981_f32,        instance.my_f32);
    assert_eq!(10.04981_f64,        instance.my_f64);
    assert_eq!([0x0100, 0x0302], instance.my_arr);
    assert_eq!(true,                          instance.my_bool);
    assert_eq!(vec![4, 5, 6],           instance.my_vec);
}
```

```rs
// Enums are supported, as long as they declare their representation.

#[derive(Serialize)]
#[repr(u16)]
enum MyEnum {
    VarA = 0,
    VarB = 1,
    VarC = 65534,
}

fn serialize_enum() {
    let mut serialized_instance = Vec::new();

    MyEnum::VarA.serialize(&mut serialized_instance).unwrap();
    MyEnum::VarC.serialize(&mut serialized_instance).unwrap();
    MyEnum::VarB.serialize(&mut serialized_instance).unwrap();

    let expected_bytes: &[u8] = &[
        0x00, 0x00,
        0xFE, 0xFF,
        0x01, 0x00,
    ];

    assert_eq!(expected_bytes, serialized_instance);
}
```
