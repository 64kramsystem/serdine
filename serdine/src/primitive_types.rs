use std::convert::TryInto;
use std::fmt::Debug;

use crate::macros::impl_for_numeric;
use crate::{Deserialize, Serialize};

impl_for_numeric!(
    Deserialize,
    i8,
    i16,
    i32,
    i64,
    i128,
    u8,
    u16,
    u32,
    u64,
    u128,
    f32,
    f64
);

impl_for_numeric!(Serialize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl Deserialize for bool {
    fn deserialize<R: std::io::Read>(mut r: R) -> Self {
        let mut buffer = [0; 1];
        r.read_exact(&mut buffer).unwrap();
        buffer[0] != 0
    }
}

impl Serialize for bool {
    fn serialize<W: std::io::Write>(&self, mut w: W) {
        let buffer = [if *self { 1 } else { 0 }; 1];
        w.write_all(&buffer).unwrap();
    }
}

impl<T, const N: usize> Deserialize for [T; N]
where
    T: Deserialize + Debug,
{
    fn deserialize<R: std::io::Read>(mut r: R) -> Self {
        // Optimization (e.g. via `arr_macro` crate) is insignificant in this context, and it should
        // be measured first, even if it was.
        //
        std::iter::repeat_with(|| T::deserialize(&mut r))
            .take(N)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

impl<T, const N: usize> Serialize for [T; N]
where
    T: Serialize,
{
    fn serialize<W: std::io::Write>(&self, mut w: W) {
        for instance in self {
            instance.serialize(&mut w);
        }
    }
}
