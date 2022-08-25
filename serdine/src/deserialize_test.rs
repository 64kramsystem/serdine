use std::{fs::File, io::Read, path::Path};

use crate::{self as serdine, Deserialize};
use serdine_derive::Deserialize;

#[derive(Deserialize)]
pub struct Mytype {
    pub my_i16: i16,
    pub my_u32: u32,
    pub my_f32: f32,
    pub my_f64: f64,
    #[deserialize = "deserialize_vec"]
    pub my_vec: Vec<u8>,
}

fn deserialize_vec<R: Read>(mut r: R) -> Vec<u8> {
    let mut buffer = Vec::new();
    r.read_to_end(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_deserialize() {
    let data_file_path =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("test_data/deserialize.dat");

    let data_file = File::open(data_file_path).unwrap();

    let instance = Mytype::deserialize(&data_file);

    assert_eq!(0x80, instance.my_i16);
    assert_eq!(0xCAFEBABE, instance.my_u32);
    assert_eq!(1004.981_f32, instance.my_f32);
    assert_eq!(10.04981_f64, instance.my_f64);
    assert_eq!(vec![0_u8, 1, 2, 3, 4], instance.my_vec);
}
