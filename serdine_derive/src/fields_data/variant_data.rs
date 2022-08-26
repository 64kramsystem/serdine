use proc_macro2::Ident;
use syn::{self, LitInt};

pub struct VariantData {
    pub variant: Ident,
    pub discriminant: LitInt,
}

impl VariantData {
    pub fn new(variant: Ident, discriminant: LitInt) -> Self {
        Self {
            variant,
            discriminant,
        }
    }
}
