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

mod builtin_types;
mod deserialize;
mod macros;
mod serialize;

pub use deserialize::Deserialize;
pub use serialize::Serialize;

pub use serdine_derive as derive;
