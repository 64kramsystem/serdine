use std::{fs::File, io::Read, path::Path};

use crate::{self as serdine, Deserialize};
use serdine_derive::Deserialize;

#[derive(Deserialize)]
pub struct Pictype {
    pub width: i16,
    pub shapeptr: u32,
    #[deserialize = "deserialize_sounddata"]
    pub sounddata: Vec<u8>,
}

fn deserialize_sounddata<R: Read>(mut r: R) -> Vec<u8> {
    let mut buffer = Vec::new();
    r.read_to_end(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_sort() {
    let test_file_path =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("test_data/deserialize.dat");

    let test_file = File::open(test_file_path).unwrap();

    let instance = Pictype::deserialize(&test_file);

    assert_eq!(0x80, instance.width);
    assert_eq!(0xCAFEBABE, instance.shapeptr);
    assert_eq!(vec![0_u8, 1, 2, 3, 4], instance.sounddata);
}
