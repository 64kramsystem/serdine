use syn::LitStr;

use crate::fields_data::NamedFieldData;

const DESERIALIZE_ATTR: &str = "deserialize";
const SERIALIZE_ATTR: &str = "serialize";

pub enum Target {
    ForSerialization,
    ForDeserialization,
}

impl Target {
    pub fn attribute_name(&self) -> &str {
        match self {
            Target::ForSerialization => SERIALIZE_ATTR,
            Target::ForDeserialization => DESERIALIZE_ATTR,
        }
    }

    pub fn set_serialization_fn(&self, field_data: &mut NamedFieldData, fn_name: LitStr) {
        match self {
            Target::ForSerialization => field_data.serialization_fn = Some(fn_name),
            Target::ForDeserialization => field_data.deserialization_fn = Some(fn_name),
        }
    }
}
