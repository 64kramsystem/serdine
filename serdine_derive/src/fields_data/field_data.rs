use proc_macro2::Ident;
use syn::{self, LitStr};

pub struct NamedFieldData {
    pub field: Ident,
    pub deserialization_fn: Option<LitStr>,
    pub serialization_fn: Option<LitStr>,
}

impl NamedFieldData {
    pub fn new(field: Ident) -> Self {
        Self {
            field,
            deserialization_fn: None,
            serialization_fn: None,
        }
    }
}
