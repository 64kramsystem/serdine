#![deny(clippy::all)]
#![allow(
  // style includes the useful `redundant_closure`
  clippy::assign_op_pattern,
  clippy::collapsible_else_if,
  clippy::collapsible_if,
  clippy::comparison_chain,
  clippy::derive_partial_eq_without_eq,
  clippy::len_zero,
  clippy::manual_range_contains,
  clippy::new_without_default,
  clippy::too_many_arguments,
  clippy::type_complexity,
)]

mod bail;
mod deserialize;
mod fields_data;
mod serialize;

use deserialize::impl_deserialize;
use proc_macro::TokenStream;
use serialize::impl_serialize;

#[proc_macro_derive(Deserialize, attributes(deserialize))]
pub fn deserialize(input: TokenStream) -> TokenStream {
    let deserialize_impl = impl_deserialize(input);

    deserialize_impl
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Serialize, attributes(serialize))]
pub fn serialize(input: TokenStream) -> TokenStream {
    let serialize_impl = impl_serialize(input);

    serialize_impl
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
