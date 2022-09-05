use crate::collection::{collect_variants_data, find_type_numeric_repr};
use crate::fields_data::{NamedFieldData, VariantData};
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
        Data::Enum(data_enum) => {
            let enum_repr = find_type_numeric_repr(&ast)?;
            let variants_data = collect_variants_data(data_enum)?;
            impl_trait_with_enum_variants(type_name, enum_repr, variants_data)?
        }
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
                quote! { #deserialization_fn(&mut r)? }
            } else {
                quote! { serdine::Deserialize::deserialize(&mut r)? }
            };

            quote! { let #field = #quoted_deserialization_fn; }
        },
    );

    let self_fields = fields_data
        .iter()
        .map(|NamedFieldData { field, .. }| quote! { #field, });

    Ok(quote!(
        impl serdine::Deserialize for #type_name {
            fn deserialize<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
                #(#fields_deserialization)*

                let result = Self {
                    #(#self_fields)*
                };

                Ok(result)
            }
        }
    ))
}

fn impl_trait_with_enum_variants(
    type_name: &Ident,
    enum_repr: Ident,
    variants_data: Vec<VariantData>,
) -> syn::Result<TokenStream2> {
    let field_matches = variants_data.iter().map(
        |VariantData {
             variant,
             discriminant,
         }| {
            quote! { #discriminant => Self::#variant, }
        },
    );

    Ok(quote!(
        impl serdine::Deserialize for #type_name {
            fn deserialize<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
                let mut buffer = [0; std::mem::size_of::<Self>()];

                r.read_exact(&mut buffer)?;

                let result = match #enum_repr::from_le_bytes(buffer) {
                    #(#field_matches)*
                    value => panic!("Unrecognized value for 'MyEnum' variant: {}", value),
                };

                Ok(result)
            }
        }
    ))
}
