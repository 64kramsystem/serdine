use std::io::Read;

pub trait Deserialize: Sized {
    fn deserialize<R: Read>(r: R) -> Result<Self, std::io::Error>;
}
