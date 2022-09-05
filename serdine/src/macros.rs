macro_rules! impl_for_numeric {
    ( Deserialize, $( $type:ty ),+ ) => {
        $(
            impl crate::Deserialize for $type {
                fn deserialize<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
                    let mut buffer = [0; std::mem::size_of::<$type>()];
                    r.read_exact(&mut buffer)?;
                    let result = <$type>::from_le_bytes(buffer);
                    Ok(result)
                }
            }
        )+
    };
    ( Serialize, $( $type:ty ),+ ) => {
        $(
            impl crate::Serialize for $type {
                fn serialize<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
                    let encoded = self.to_le_bytes();
                    w.write_all(&encoded)?;
                    Ok(())
                }
            }
        )+
    };
}

pub(crate) use impl_for_numeric;
