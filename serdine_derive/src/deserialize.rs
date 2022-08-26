use crate::fields_data::NamedFieldData;
use crate::target::Target::ForDeserialization;
use crate::{bail::bail, collection::collect_named_fields_data};

use proc_macro2::Ident;
use quote::quote;
use syn::{self, parse2, Data, DataStruct, DeriveInput, Fields};

type TokenStream2 = proc_macro2::TokenStream;

pub(crate) fn impl_deserialize(input: impl Into<TokenStream2>) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = parse2(input.into())?;
    let type_name = &ast.ident;

    let deserialize_impl = match &ast.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(fields) => {
                let named_fields_data = collect_named_fields_data(fields, ForDeserialization)?;
                impl_trait_with_named_fields(type_name, named_fields_data)?
            }
            Fields::Unnamed(_) => bail!("Unnamed fields not supported!"),
            Fields::Unit => bail!("Unit fields not supported!"),
        },
        Data::Enum(_) => bail!("Enums not supported!"),
        Data::Union(_) => bail!("Unions not supported!"),
    };

    Ok(quote!(
        #deserialize_impl
    ))
}

fn impl_trait_with_named_fields(
    type_name: &Ident,
    fields_data: Vec<NamedFieldData>,
) -> syn::Result<TokenStream2> {
    let fields_deserialization = fields_data.iter().map(
        |NamedFieldData {
             field,
             deserialization_fn,
             ..
         }| {
            let quoted_deserialization_fn = if let Some(deserialization_fn) = deserialization_fn {
                let deserialization_fn =
                    Ident::new(&deserialization_fn.value(), deserialization_fn.span());
                quote! { #deserialization_fn(&mut r) }
            } else {
                quote! { serdine::Deserialize::deserialize(&mut r) }
            };

            quote! { let #field = #quoted_deserialization_fn; }
        },
    );

    let self_fields = fields_data
        .iter()
        .map(|NamedFieldData { field, .. }| quote! { #field, });

    Ok(quote!(
        impl serdine::Deserialize for #type_name {
            fn deserialize<R: std::io::Read>(mut r: R) -> Self {
                #(#fields_deserialization)*

                Self {
                    #(#self_fields)*
                }
            }
        }
    ))
}
